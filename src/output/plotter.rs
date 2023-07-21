use std::collections::BTreeMap;
use std::error::Error;
use chrono::NaiveDateTime;

use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, VPos};
use plotters_backend::{
    BackendColor, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
};

use crate::analyzer::types::PowerAdded;

#[derive(Copy, Clone)]
enum PixelState {
    Empty,
    HLine,
    VLine,
    Cross,
    Pixel,
    Text(char),
    Circle(bool),
}

impl PixelState {
    fn to_char(self) -> char {
        match self {
            Self::Empty => ' ',
            Self::HLine => '-',
            Self::VLine => '|',
            Self::Cross => '+',
            Self::Pixel => '.',
            Self::Text(c) => c,
            Self::Circle(filled) => {
                if filled {
                    '@'
                } else {
                    'O'
                }
            }
        }
    }

    fn update(&mut self, new_state: PixelState) {
        let next_state = match (*self, new_state) {
            (Self::HLine, Self::VLine) => Self::Cross,
            (Self::VLine, Self::HLine) => Self::Cross,
            (_, Self::Circle(what)) => Self::Circle(what),
            (Self::Circle(what), _) => Self::Circle(what),
            (_, Self::Pixel) => Self::Pixel,
            (Self::Pixel, _) => Self::Pixel,
            (_, new) => new,
        };

        *self = next_state;
    }
}

struct TextDrawingBackend(Vec<PixelState>);

impl DrawingBackend for TextDrawingBackend {
    type ErrorType = std::io::Error;

    fn get_size(&self) -> (u32, u32) {
        (100, 30)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<std::io::Error>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<std::io::Error>> {
        for r in 0..30 {
            let mut buf = String::new();
            for c in 0..100 {
                buf.push(self.0[r * 100 + c].to_char());
            }
            println!("{}", buf);
        }

        Ok(())
    }

    fn draw_pixel(
        &mut self,
        pos: (i32, i32),
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<std::io::Error>> {
        if color.alpha > 0.3 {
            self.0[(pos.1 * 100 + pos.0) as usize].update(PixelState::Pixel);
        }
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if from.0 == to.0 {
            let x = from.0;
            let y0 = from.1.min(to.1);
            let y1 = from.1.max(to.1);
            for y in y0..y1 {
                self.0[(y * 100 + x) as usize].update(PixelState::VLine);
            }
            return Ok(());
        }

        if from.1 == to.1 {
            let y = from.1;
            let x0 = from.0.min(to.0);
            let x1 = from.0.max(to.0);
            for x in x0..x1 {
                self.0[(y * 100 + x) as usize].update(PixelState::HLine);
            }
            return Ok(());
        }

        plotters_backend::rasterizer::draw_line(self, from, to, style)
    }

    fn estimate_text_size<S: BackendTextStyle>(
        &self,
        text: &str,
        _: &S,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        Ok((text.len() as u32, 1))
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: (i32, i32),
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let (width, height) = self.estimate_text_size(text, style)?;
        let (width, height) = (width as i32, height as i32);
        let dx = match style.anchor().h_pos {
            HPos::Left => 0,
            HPos::Right => -width,
            HPos::Center => -width / 2,
        };
        let dy = match style.anchor().v_pos {
            VPos::Top => 0,
            VPos::Center => -height / 2,
            VPos::Bottom => -height,
        };
        let offset = (pos.1 + dy).max(0) * 100 + (pos.0 + dx).max(0);
        for (idx, chr) in (offset..).zip(text.chars()) {
            self.0[idx as usize].update(PixelState::Text(chr));
        }
        Ok(())
    }
}

pub fn draw_chart(
    data: BTreeMap<i64, PowerAdded>
) -> Result<(), Box<dyn Error>>
{
    let b = TextDrawingBackend(vec![PixelState::Empty; 5000]).into_drawing_area();

    //let min_key = data.keys().next().map(|v| v).unwrap();
    let min_key = NaiveDateTime::parse_from_str("2022-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().timestamp();
    let max_key = data.keys().last().unwrap();

    let mut chart = ChartBuilder::on(&b)
        .margin(1)
        .caption("Added Units by Day since 2022", ("sans-serif", (10).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (5i32).percent_width())
        .set_label_area_size(LabelAreaPosition::Bottom, (10i32).percent_height())
        .build_cartesian_2d(min_key..*max_key, 0..10)?; //@TODO

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_label_formatter(&|x| NaiveDateTime::from_timestamp_opt(*x, 0).unwrap().date().format("%Y%m%d").to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(
        data.iter().map(|point| (*point.0, point.1.0 as i32)),
        RED,
    ))?;

    b.present()?;

    Ok(())
}