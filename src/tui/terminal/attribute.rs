use crossterm::style::Color;

use super::super::AnnotationType;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 150,
                    g: 220,
                    b: 130,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 60,
                    g: 150,
                    b: 30,
                }),
            },
            AnnotationType::Selected => Self {
                foreground: None,
                background: Some(Color::Rgb {
                    r: 60,
                    g: 55,
                    b: 30,
                }),
            },
        }
    }
}
