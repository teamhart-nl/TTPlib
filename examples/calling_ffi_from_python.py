#Make sure you compiled TTP
from ctypes import * 
import os
dir_path =os.path.abspath(os.path.join(os.path.realpath( __file__ ), '..','..', 'target','debug'))
if os.name == 'nt':
    mydll = cdll.LoadLibrary(dir_path + "/TTP.dll")
else:
    mydll = cdll.LoadLibrary(dir_path + "/libTTP.so")  
word_to_phonemes = mydll.word_to_phonemes 
word_to_phonemes.argtypes = [c_char_p]
word_to_phonemes.restype = c_char_p   
while True:
    word = input("Enter a word: ")
    print(word_to_phonemes(word.encode('utf-8').lower()).decode('utf-8'))

