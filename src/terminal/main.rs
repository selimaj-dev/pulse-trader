pub mod command;
pub mod formatting;
pub mod terminal;

use std::any::Any;

use chrono::{Local, Utc};
use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::{Refresh, State},
    unit::Size,
    widget::{
        align::End,
        input::{Input, InputState},
        outline::{Outline, VLine},
        scroll::{ScrollState, ScrollText},
        spaced::SpacedColumns,
    },
};

use crate::formatting::{Formatted, apply_padding};

use pulse_wire::terminal::{
    ActivePosition, EventLog, InspectTarget, MarketOverview, Signal, Status, WatchListItem,
};

pub struct PulseTradeApp {
    command: State<InputState>,
    scroll: State<ScrollState<7>>,
    watch_list: State<Vec<WatchListItem>>,
    active_positions: State<Vec<ActivePosition>>,
    logs: State<Vec<EventLog>>,
    signals: State<Vec<Signal>>,
    market_overview: State<MarketOverview>,
    status: State<Status>,
    inspect: State<InspectTarget>,
}

impl App for PulseTradeApp {
    async fn init(&mut self, _ctx: &pulse_ui::state::Context) {}

    async fn update(&mut self, ctx: &pulse_ui::state::Context, event: Box<dyn Any + Send + Sync>) {
        if let Some(Refresh) = event.downcast_ref() {
            return;
        }

        if let Some(event) = event.downcast_ref() {
            if self.command.value.lock().await.handle_event(event) {
                return;
            } else if let crossterm::event::Event::Key(key) = event {
                if key.code.is_enter() {
                    let mut command = self.command.lock().await;
                    let command_text = command.text.clone();
                    command.cursor = 0;
                    command.text.clear();

                    drop(command);
                    self.execute_command(ctx, command_text.trim()).await;
                } else if key.code.is_up() {
                    self.scroll.value.lock().await.up();
                } else if key.code.is_down() {
                    self.scroll.value.lock().await.down();
                } else if key.code.is_back_tab() {
                    self.scroll.value.lock().await.back_tab();
                } else if key.code.is_tab() {
                    self.scroll.value.lock().await.tab();
                }
            }
        }
    }

    async fn layout(&self) -> pulse_ui::layout::LayoutItem {
        LayoutItem::Frame {
            padding: 1,
            item: Box::new(layout(vec![
                LayoutItem::Columns {
                    unit: Size::Fixed(1),
                    items: vec![
                        LayoutItem::Widget(Size::Flex(1)),
                        LayoutItem::Widget(Size::Flex(1)),
                        LayoutItem::Widget(Size::Flex(1)),
                    ],
                },
                // Account status
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Flex(1)),
                // Live status
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Flex(1)),
                // Event logs
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Fill),
                // Input
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Fixed(1)),
            ])),
        }
    }

    async fn render(&mut self, layout: pulse_ui::layout::Allocation) {
        layout.draw_frame(0, Outline);
        layout.draw(
            0,
            format!("   PULSE TRADER v{}", env!("CARGO_PKG_VERSION")),
        );
        // layout.draw(1, Center("LIVE".to_string()));
        layout.draw(
            2,
            End(format!(
                "{} UTC ({} Local)",
                Utc::now().format("%H:%M"),
                Local::now().format("%H:%M")
            )),
        );
        layout.draw_frame(3, VLine);
        layout.draw_frame(4, VLine);
        layout.draw_frame(5, VLine);
        layout.draw_frame(6, VLine);

        layout.draw(
            3,
            SpacedColumns(vec![
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(advanced_draw(&self.scroll, 0, "WATCH LIST", &self.watch_list).await),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        advanced_draw(&self.scroll, 1, "ACTIVE POSITIONS", &self.active_positions)
                            .await,
                    ),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        advanced_draw(&self.scroll, 2, "MARKET OVERVIEW", &self.market_overview)
                            .await,
                    ),
                ),
            ]),
        );

        layout.draw(
            4,
            SpacedColumns(vec![
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(advanced_draw(&self.scroll, 3, "SIGNALS", &self.signals).await),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(advanced_draw(&self.scroll, 4, "INSPECTOR", &self.inspect).await),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(advanced_draw(&self.scroll, 5, "STATUS", &self.status).await),
                ),
            ]),
        );

        layout.draw(
            5,
            advanced_draw(&self.scroll, 6, "EVENT LOGS", &self.logs).await,
        );

        layout.draw(6, Input(" > ", &*self.command.lock().await));
    }
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut client = terminal::TerminalClient::new().await?;

    client
        .send(pulse_wire::terminal::TerminalClientMessage::ExecuteCommand(
            "Hello".to_string(),
        ))
        .await?;

    pulse_ui::run(|ctx| {
        client.use_app(PulseTradeApp {
            command: ctx.use_state(InputState::new()),
            scroll: ctx.use_state(ScrollState(1, [0; 7])),
            watch_list: ctx.use_state(Vec::new()),
            active_positions: ctx.use_state(Vec::new()),
            signals: ctx.use_state(Vec::new()),
            logs: ctx.use_state(Vec::new()),
            inspect: ctx.use_state(InspectTarget::None),
            market_overview: ctx.use_state(MarketOverview {
                trend: pulse_wire::terminal::MarketTrend::Bullish,
                volatility: pulse_wire::terminal::Volatility::High,
                pressure: 0.324,
                alerts: Vec::new(),
            }),
            status: ctx.use_state(Status {
                feed: pulse_wire::terminal::Feed::Connected,
                exchange: "Binance".to_string(),
                dex: "DEX SCREENER".to_string(),
                latency: 18,
            }),
        })
    })
    .await;

    Ok(())
}

pub async fn advanced_draw<const N: usize, T: Formatted>(
    scroll: &State<ScrollState<N>>,
    index: usize,
    title: &'static str,
    state: &State<T>,
) -> ScrollText {
    let scroll = scroll.lock().await;

    scroll.scroll(
        index,
        format!(" {}{title}\x1b[0m", scroll.get_selected(index)),
        apply_padding(state.lock().await.get_formatted()).join("\n"),
    )
}
