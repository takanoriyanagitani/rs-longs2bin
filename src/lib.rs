use std::io;

use io::Write;

pub struct LongWriter<W, C> {
    pub writer: W,
    pub long2dat: C,
}

impl<W, C> LongWriter<W, C>
where
    W: Write,
    C: Fn(i64) -> [u8; 8],
{
    pub fn into_long_consumer(mut self) -> impl FnMut(i64) -> Result<(), io::Error> {
        move |lng: i64| {
            let dat: [u8; 8] = (self.long2dat)(lng);
            self.writer.write_all(&dat)?;
            Ok(())
        }
    }
}

pub enum Mode {
    Le,
    Be,
}

impl Mode {
    pub fn lngs2writer<W, I>(&self, lngs: I, wtr: W) -> Result<(), io::Error>
    where
        W: Write,
        I: Iterator<Item = Result<i64, io::Error>>,
    {
        let lng2dat = match self {
            Self::Le => |lng: i64| lng.to_le_bytes(),
            Self::Be => |lng: i64| lng.to_be_bytes(),
        };

        let lng2wtr = LongWriter {
            writer: wtr,
            long2dat: lng2dat,
        };

        let mut lng_consumer = lng2wtr.into_long_consumer();

        for rlng in lngs {
            let lng: i64 = rlng?;
            lng_consumer(lng)?;
        }

        Ok(())
    }
}

impl std::str::FromStr for Mode {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "le" => Ok(Self::Le),
            "be" => Ok(Self::Be),
            _ => Err(io::Error::other(format!("unknown mode: {s}"))),
        }
    }
}
