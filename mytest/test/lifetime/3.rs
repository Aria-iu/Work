fn bar(writer: &mut Writer) {
    baz(&mut writer.indent());
    writer.write("world");
}

fn baz(writer: &mut Writer) {
    writer.write("hello");
}

pub struct Writer<'a> {
    target: &'a mut String,
    indent: usize,
}

impl<'a> Writer<'a> {
    fn indent<'b>(&'b mut self) -> Writer<'b> where 'a:'b{
        Writer {
            target: self.target,
            indent: self.indent + 1,
        }
    }

    fn write<'b>(&'b mut self, s: &'b str)where 'a:'b {
        for _ in 0..self.indent {
            self.target.push(' ');
        }
        self.target.push_str(s);
        self.target.push('\n');
    }
}

fn main() {}