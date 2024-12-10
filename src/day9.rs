use super::read_digits;

pub fn day9_a() {
    let files = read_digits(9);
    let mut first = 0;
    let mut first_counter = files[first];
    let mut first_id = 0;
    let mut last = files.len() - 1;
    let mut last_counter = files[last];
    let mut last_id = last / 2;
    let mut position = 0;
    let mut result = 0;
    while first < last {
        while first_counter > 0 {
            result += position * first_id;
            position += 1;
            first_counter -= 1;
        }
        first += 1;
        first_counter = files[first];
        while first_counter > 0 {
            if last_counter == 0 {
                last -= 2;
                last_counter = files[last];
                last_id -= 1;
            }
            result += position * last_id;
            position += 1;
            first_counter -= 1;
            last_counter -= 1;
        }
        first += 1;
        first_counter = files[first];
        first_id += 1;
    }
    while last_counter != 0 {
        result += position * last_id;
        position += 1;
        last_counter -= 1;
    }
    println!("{result}");
}

#[derive(Copy, Clone)]
struct File {
    id: usize,
    length: usize,
}

impl File {
    fn count(&self, position: &mut usize, result: &mut usize) {
        for _ in 0..self.length {
            *result += *position * self.id;
            *position += 1;
        }
    }
}

struct FileSlot {
    file: File,
    moved: bool,
}

impl FileSlot {
    const fn new(id: usize, length: usize) -> Self {
        Self { file: File { id, length }, moved: false }
    }

    fn count(&self, position: &mut usize, result: &mut usize) {
        if self.moved {
            *position += self.file.length;
        } else {
            self.file.count(position, result);
        }
    }
}

struct Empty {
    left: usize,
    moved: Vec<File>,
}

impl Empty {
    const fn new(length: usize) -> Self { Self { left: length, moved: Vec::new() } }

    fn push(&mut self, file: File) {
        self.left -= file.length;
        self.moved.push(file);
    }

    fn count(&self, position: &mut usize, result: &mut usize) {
        for f in &self.moved {
            f.count(position, result);
        }
        *position += self.left;
    }
}

pub fn day9_b() {
    let files = read_digits(9);
    let mut file_slots = Vec::new();
    let mut empty = Vec::new();
    for (i, l) in files.iter().enumerate() {
        if i % 2 == 0 {
            file_slots.push(FileSlot::new(i / 2, *l as _));
        } else {
            empty.push(Empty::new(*l as _));
        }
    }
    empty.push(Empty { left: 0, moved: Vec::new() });
    for (i, f) in file_slots.iter_mut().enumerate().rev() {
        if let Some(e) = empty[..i].iter_mut().find(|e| e.left >= f.file.length) {
            e.push(f.file);
            f.moved = true;
        }
    }
    let mut position = 0;
    let mut result = 0;
    for (f, e) in file_slots.iter().zip(empty.iter()) {
        f.count(&mut position, &mut result);
        e.count(&mut position, &mut result);
    }
    println!("{result}");
}
