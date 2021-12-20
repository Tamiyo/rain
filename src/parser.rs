use crate::interpreter::Instruction;

const COLOR_PAD: [u8; 3] = [0, 0, 0];

pub(crate) fn parse(pixels: &[u8]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut i: usize = 0;

    while i < pixels.len() - 2 {
        let color = [pixels[i], pixels[i + 1], pixels[i + 2]];
        let mut command_length: u8 = 0;

        while i < pixels.len() - 2
            && pixels[i] == COLOR_PAD[0]
            && pixels[i + 1] == COLOR_PAD[1]
            && pixels[i + 2] == COLOR_PAD[2]
        {
            i += 3;
        }

        // get the next command
        while i < pixels.len() - 2
            && pixels[i] == color[0]
            && pixels[i + 1] == color[1]
            && pixels[i + 2] == color[2]
        {
            command_length += 1;
            i += 3
        }

        match command_length {
            1 => instructions.push(Instruction::IncrementPtr),
            2 => instructions.push(Instruction::DecrementPtr),
            3 => instructions.push(Instruction::IncrementValueAtPtr),
            4 => instructions.push(Instruction::DecrementValueAtPtr),
            5 => instructions.push(Instruction::PutChar),
            6 => instructions.push(Instruction::GetChar),
            7 => instructions.push(Instruction::WhileBegin),
            8 => instructions.push(Instruction::WhileEnd),
            _ => (),
        }
    }
    instructions.push(Instruction::Exit);
    instructions
}
