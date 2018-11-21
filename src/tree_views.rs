

use termion::{style, color};
use std::io::{self, Write};
use app::App;
use flat_tree::{TreeLine, Tree, LineType};

pub trait TreeView {
    fn tree_height(&self) -> u16;
    fn write_tree(&mut self, tree: &Tree) -> io::Result<()>;
    fn write_line_key(&mut self, line: &TreeLine, selected: bool) -> io::Result<()>;
    fn write_line_name(&mut self, line: &TreeLine) -> io::Result<()>;
}

impl TreeView for App {
    fn tree_height(&self) -> u16 {
        self.h - 2
    }
    fn write_tree(&mut self, tree: &Tree) -> io::Result<()> {
        for y in 1..self.h-1 {
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(1, y),
                termion::clear::CurrentLine,
            )?;
            let line_index = (y -1) as usize;
            if line_index >= tree.lines.len() {
                continue;
            }
            let line = &tree.lines[line_index];
            let selected = line_index == tree.selection;
            for depth in 0..line.depth {
                write!(
                    self.stdout,
                    "{}{}{}",
                    color::Fg(color::AnsiValue::grayscale(5)),
                    match line.left_branchs[depth as usize] {
                        true    => {
                            match tree.has_branch(line_index+1, depth as usize) {
                                true    => match depth == line.depth-1 {
                                        true    => "├─",
                                        false   => "│ ",
                                },
                                false   => "└─",
                            }
                        },
                        false   => "  ",
                    },
                    color::Fg(color::Reset),
                )?;
            }
            self.write_line_key(line, selected)?;
            self.write_line_name(line)?;
            write!(
                self.stdout,
                "{}{}{}",
                style::Reset,
                color::Fg(color::Reset),
                color::Bg(color::Reset),
            )?;
        }
        self.stdout.flush()?;
        Ok(())
    }

    fn write_line_key(&mut self, line: &TreeLine, selected: bool) -> io::Result<()>{
        match &line.content {
            LineType::Pruning(n)    => {
            },
            _                       => {
                if selected {
                    write!(
                        self.stdout,
                        "{} {} {}{}",
                        color::Bg(color::AnsiValue::grayscale(5)),
                        &line.key,
                        color::Bg(color::AnsiValue::grayscale(2)),
                        termion::clear::UntilNewline,
                    )?;

                } else {
                    write!(
                        self.stdout,
                        "{}{} {} {}{}",
                        color::Bg(color::AnsiValue::grayscale(2)),
                        color::Fg(color::AnsiValue::grayscale(18)),
                        &line.key,
                        color::Fg(color::Reset),
                        color::Bg(color::Reset),
                    )?;
                }
            },
        }
        Ok(())
    }

    fn write_line_name(&mut self, line: &TreeLine) -> io::Result<()> {
        match &line.content {
            LineType::Dir(name)        => {
                write!(
                    self.stdout,
                    " {}{}",
                    style::Bold,
                    &name,
                )?;
            },
            LineType::File(name)        => {
                write!(
                    self.stdout,
                    " {}",
                    &name,
                )?;
            },
            LineType::Pruning(n)  => {
                write!(
                    self.stdout,
                    "{} ... {} other files…",
                    style::Italic,
                    n,
                )?;
            },
        }
        Ok(())
    }

}