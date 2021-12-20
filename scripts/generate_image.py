import numpy as np
import random

from enum import Enum
from itertools import cycle
from PIL import Image

class Command(Enum):
    INCREMENT_PTR = 1
    DECREMENT_PTR = 2
    INCREMENT_VALUE_AT_PTR = 3
    DECREMENT_VALUE_AT_PTR = 4
    PUT_CHAR = 5
    GET_CHAR = 6
    WHILE_BEGIN = 7
    WHILE_END = 8


COMMAND_MAP = {
    '>': Command.INCREMENT_PTR,
    '<': Command.DECREMENT_PTR,
    '+': Command.INCREMENT_VALUE_AT_PTR,
    '-': Command.DECREMENT_VALUE_AT_PTR,
    '.': Command.PUT_CHAR,
    ',': Command.GET_CHAR,
    '[': Command.WHILE_BEGIN,
    ']': Command.WHILE_END
}

PIXEL_MIN = 1
PIXEL_MAX = 255

PAD_COLOR = [0, 0, 0]

def __generate_image_from_commands__(input: str):
    def get_color():
        return [random.randint(PIXEL_MIN, PIXEL_MAX), random.randint(PIXEL_MIN, PIXEL_MAX), random.randint(PIXEL_MIN, PIXEL_MAX)]
    
    pixel_arr = []
    colors = cycle([get_color(), get_color(), get_color(), get_color()])
    
    # form the pixel values for the input image
    for ch in input:
        command = COMMAND_MAP[ch]
        color = next(colors)
        pixel_arr += [color] * command.value
    
    # determine what perfect square will fit the image
    ns = np.ceil(np.sqrt(len(pixel_arr))).astype(int)
    
    # add padding
    pad_amount = round(ns ** 2 - len(pixel_arr))
    pixel_arr += [[0, 0, 0]] * pad_amount
    
    # convert to PIL image
    pixel_arr = np.array([np.array(x).astype(np.uint8) for x in pixel_arr]).reshape((400, 400, 3))
    img = Image.fromarray(pixel_arr)
    img.save('images/hello_world.png')

if __name__ == '__main__':
    """
        Generate @ https://andrew.hedges.name/experiments/brainf_cker/#
        Minify @ https://copy.sh/brainfuck/
    """
    with open('scripts/input.txt') as f:
        input = ''.join(f.readlines())
        __generate_image_from_commands__(input)
