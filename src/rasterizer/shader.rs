use kd_tree::{KdPoint, KdTree, KdTreeN};
use typenum::U9;

pub trait Shader<const SIZE: usize> {
    fn shade_to_char(&self, shade: &[f32; SIZE]) -> char;
}

pub struct SimpleShader;

impl SimpleShader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Shader<1> for SimpleShader {
    fn shade_to_char(&self, shade: &[f32; 1]) -> char {
        let shade = shade[0];
        if shade <= 0.05 {
            ' '
        } else if shade <= 0.20 {
            '.'
        } else if shade <= 0.30 {
            ':'
        } else if shade <= 0.40 {
            '-'
        } else if shade <= 0.50 {
            '='
        } else if shade <= 0.60 {
            '+'
        } else if shade <= 0.70 {
            '*'
        } else if shade <= 0.80 {
            '#'
        } else if shade <= 0.90 {
            '%'
        } else if shade <= 1.0 {
            '@'
        } else {
            ' '
        }
    }
}

pub struct UnicodeShader(KdTreeN<Shade, U9>);

impl UnicodeShader {
    pub fn new() -> Self {
        let unicode_chars: KdTreeN<Shade, U9> = KdTree::build_by_ordered_float(vec![
            // full blocks
            Shade {
                ch: ' ',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '░',
                p: [0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2],
            },
            Shade {
                ch: '▒',
                p: [0.45, 0.45, 0.45, 0.45, 0.45, 0.45, 0.45, 0.45, 0.45],
            },
            Shade {
                ch: '🮐',
                p: [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            },
            Shade {
                ch: '▓',
                p: [0.85, 0.85, 0.85, 0.85, 0.85, 0.85, 0.85, 0.85, 0.85],
            },
            Shade {
                ch: '█',
                p: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            },
            // triangle shades
            Shade {
                ch: '🮜',
                p: [0.5, 0.5, 0.25, 0.5, 0.25, 0.0, 0.25, 0.0, 0.0],
            },
            Shade {
                ch: '🮝',
                p: [0.25, 0.5, 0.5, 0.0, 0.25, 0.5, 0.0, 0.0, 0.25],
            },
            Shade {
                ch: '🮞',
                p: [0.0, 0.0, 0.25, 0.0, 0.25, 0.5, 0.25, 0.5, 0.5],
            },
            Shade {
                ch: '🮟',
                p: [0.25, 0.0, 0.0, 0.5, 0.25, 0.0, 0.5, 0.5, 0.25],
            },
            // shaded halves
            Shade {
                ch: '🮏',
                p: [0.0, 0.0, 0.0, 0.2, 0.2, 0.2, 0.4, 0.4, 0.4],
            },
            Shade {
                ch: '🮎',
                p: [0.4, 0.4, 0.4, 0.2, 0.2, 0.2, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🮌',
                p: [0.5, 0.25, 0.0, 0.5, 0.25, 0.0, 0.5, 0.25, 0.0],
            },
            Shade {
                ch: '🮍',
                p: [0.0, 0.25, 0.5, 0.0, 0.25, 0.5, 0.0, 0.25, 0.25],
            },
            Shade {
                ch: '🮑',
                p: [1.0, 1.0, 1.0, 0.8, 0.8, 0.8, 0.6, 0.6, 0.6],
            },
            Shade {
                ch: '🮒',
                p: [0.6, 0.6, 0.6, 0.8, 0.8, 0.8, 1.0, 1.0, 1.0],
            },
            Shade {
                ch: '▌',
                p: [1.0, 0.5, 0.0, 1.0, 0.5, 0.0, 1.0, 0.5, 0.0],
            },
            Shade {
                ch: '▐',
                p: [0.0, 0.5, 1.0, 0.0, 0.5, 1.0, 0.0, 0.5, 1.0],
            },
            // full corner triangles
            Shade {
                ch: '🭗',
                p: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🭢',
                p: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🭇',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            },
            Shade {
                ch: '🬼',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            },
            // side triangles
            Shade {
                ch: '🢑',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            },
            Shade {
                ch: '🢓',
                p: [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🞀',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🢒',
                p: [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            // big side triangles
            Shade {
                ch: '🭯',
                p: [0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.5, 1.0, 0.5],
            },
            Shade {
                ch: '🭭',
                p: [0.5, 1.0, 0.5, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🭮',
                p: [0.0, 0.0, 0.5, 0.0, 0.5, 1.0, 0.0, 0.0, 0.5],
            },
            Shade {
                ch: '🭬',
                p: [0.5, 0.0, 0.0, 1.0, 0.5, 0.0, 0.5, 0.0, 0.0],
            },
            // other
            Shade {
                ch: '▂',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            },
            Shade {
                ch: '🬰',
                p: [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            },
            Shade {
                ch: '🮔',
                p: [0.6, 0.6, 0.6, 0.8, 0.8, 0.8, 1.0, 1.0, 1.0],
            },
        ]);
        Self(unicode_chars)
    }
}

impl Shader<9> for UnicodeShader {
    fn shade_to_char(&self, shade: &[f32; 9]) -> char {
        self.0.nearest(shade).unwrap().item.ch
    }
}

pub struct Shade {
    p: [f32; 9],
    ch: char,
}

impl KdPoint for Shade {
    type Scalar = f32;
    type Dim = typenum::U9;
    fn at(&self, k: usize) -> f32 {
        self.p[k]
    }
}
