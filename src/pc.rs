pub trait PulseCom {
    fn to_com(&self) -> Vec<u8>;
    fn from_com(_com: &mut Vec<u8>) -> Self;
}

impl<T: PulseCom> PulseCom for Vec<T> {
    fn to_com(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&(self.len() as u32).to_le_bytes());

        for item in self {
            vec.extend(item.to_com());
        }

        vec
    }

    fn from_com(com: &mut Vec<u8>) -> Self {
        let len_bytes: [u8; 4] = com.drain(..4).collect::<Vec<_>>().try_into().unwrap();

        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut result = Vec::with_capacity(len);

        for _ in 0..len {
            result.push(T::from_com(com));
        }

        result
    }
}

impl PulseCom for String {
    fn to_com(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let mut out = Vec::with_capacity(4 + bytes.len());

        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);

        out
    }

    fn from_com(com: &mut Vec<u8>) -> Self {
        let len = u32::from_le_bytes(com[..4].try_into().unwrap()) as usize;
        com.drain(..4);

        let bytes: Vec<u8> = com.drain(..len).collect();
        String::from_utf8(bytes).unwrap()
    }
}

macro_rules! int_com {
    ($t:ty) => {
        impl PulseCom for $t {
            fn to_com(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }

            fn from_com(com: &mut Vec<u8>) -> Self {
                const N: usize = std::mem::size_of::<$t>();
                let bytes: [u8; N] = com.drain(..N).collect::<Vec<_>>().try_into().unwrap();
                <$t>::from_le_bytes(bytes)
            }
        }
    };
}

int_com!(i8);
int_com!(i16);
int_com!(i32);
int_com!(i64);
int_com!(isize);

int_com!(u8);
int_com!(u16);
int_com!(u32);
int_com!(u64);
int_com!(usize);

int_com!(f64);
int_com!(f32);

#[macro_export]
macro_rules! p_com {
    (struct $name:ident { $($n:ident: $v:ty),* $(,)? }) => {
        #[derive(Debug, Clone)]
        pub struct $name { $(pub $n: $v),* }

        impl PulseCom for $name {
            fn to_com(&self) -> Vec<u8> {
                let mut vec = Vec::new();
                $(vec.extend(self.$n.to_com());)*
                vec
            }

            fn from_com(com: &mut Vec<u8>) -> Self {
                Self {
                    $($n: <$v>::from_com(com),)*
                }
            }
        }
    };
}
