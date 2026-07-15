pub trait Formatted {
    fn get_formatted(&self) -> Vec<String>;
}

pub struct WatchListItem {
    pub symbol: String,
    pub price: f64,
    pub trend: f64,
}

pub struct ActivePosition {
    pub symbol: String,
    pub profit: f64,
    pub lot: f64,
}

pub struct Account {
    pub equity: f64,
    pub liquid: f64,
    pub unreal: f64,
    pub realized: f64,
    pub margin: f64,
}

impl<T: Formatted> Formatted for Vec<T> {
    fn get_formatted(&self) -> Vec<String> {
        let mut output = Vec::new();

        let rows: Vec<Vec<String>> = self.iter().map(|v| v.get_formatted()).collect();

        if rows.is_empty() {
            return output;
        }

        let column_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);

        let mut widths = vec![0; column_count];

        // Find the largest width per column
        for row in &rows {
            for (i, col) in row.iter().enumerate() {
                widths[i] = widths[i].max(col.len());
            }
        }

        // Render rows
        for (row_idx, row) in rows.iter().enumerate() {
            let mut row_out = String::new();

            for (col_idx, col) in row.iter().enumerate() {
                let width = widths[col_idx];

                // Right-align numbers, left-align text could be customized here
                row_out.push_str(&format!("{:>width$}", col, width = width));

                if col_idx + 1 != row.len() {
                    row_out.push_str("  ");
                }
            }

            if row_idx + 1 != rows.len() {
                output.push(row_out);
            }
        }

        output
    }
}

impl Formatted for WatchListItem {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            self.symbol.to_string(),
            self.price.to_string(),
            format!(
                "{} {}",
                if self.trend.is_sign_positive() {
                    "▲"
                } else {
                    "▼"
                },
                self.trend.abs()
            ),
        ]
    }
}

pub fn space_out(mut items: Vec<String>) -> Vec<String> {
    for item in &mut items {
        item.insert(0, ' ');
    }

    items
}
