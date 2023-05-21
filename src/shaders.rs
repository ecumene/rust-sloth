use kd_tree::{KdPoint, KdTree, KdTreeN};
use typenum::U9;

pub enum Shader {
    Simple,
    Unicode(KdTreeN<Shade, U9>),
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

impl Shader {
    pub fn precision(&self) -> usize {
        match self {
            Self::Simple => 1,
            Self::Unicode(_) => 3,
        }
    }

    pub fn shader_to_char(&self, shade: &[f32]) -> char {
        match self {
            Self::Simple => Shader::simple_shade(shade),
            Self::Unicode(tree) => Shader::unicode_shade(shade, tree),
        }
    }

    fn simple_shade(shade: &[f32]) -> char {
        let shade = shade[0];
        if shade <= 0.20 {
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

    fn unicode_shade(shade: &[f32], char_tree: &KdTreeN<Shade, U9>) -> char {
        let shade: &[f32; 9] = shade.try_into().unwrap();
        char_tree.nearest(shade).unwrap().item.ch
    }

    pub fn new_unicode() -> Self {
        let unicode_chars: KdTreeN<Shade, U9> = KdTree::build_by_ordered_float(vec![
            // full blocks
            Shade {
                ch: ' ',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '░',
                p: [0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25],
            },
            Shade {
                ch: '▒',
                p: [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            },
            Shade {
                ch: '🮐',
                p: [0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75],
            },
            Shade {
                ch: '▓',
                p: [0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9],
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
                p: [0.0, 0.0, 0.0, 0.25, 0.25, 0.25, 0.5, 0.5, 0.5],
            },
            Shade {
                ch: '🮎',
                p: [0.5, 0.5, 0.5, 0.25, 0.25, 0.25, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🮌',
                p: [0.5, 0.25, 0.0, 0.5, 0.25, 0.0, 0.5, 0.25, 0.0],
            },
            Shade {
                ch: '🮎',
                p: [0.5, 0.5, 0.5, 0.25, 0.25, 0.25, 0.0, 0.0, 0.0],
            },
            Shade {
                ch: '🮍',
                p: [0.0, 0.25, 0.5, 0.0, 0.25, 0.5, 0.0, 0.25, 0.25],
            },
            // other
            Shade {
                ch: '.',
                p: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0],
            },
            Shade {
                ch: '·',
                p: [0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0],
            },
        ]);
        Self::Unicode(unicode_chars)
    }
}
