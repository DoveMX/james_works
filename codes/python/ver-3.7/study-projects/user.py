#!/usr/bin/python3
# -*- coding:utf-8 -*-

import sys
from dataclasses import dataclass
from libs.fibo import fib2

@dataclass
class User():
    name:str
    user_id: int
    just_joined: bool=True
    
 
def cheeseshop(kind, *arguments, **keywords):
    """Checking arguments.
    """
    print("-- Do you have any", kind, "?")
    print("-- I'm sorry, we're all out of", kind)
    for arg in arguments:
        print(arg)
    print("-" * 40)
    for kw in keywords:
        print(kw, ":", keywords[kw])
 
 
def concat(*names, sep="/"):
    return sep.join(names)

cheeseshop("Limburger", "It's very runny, sir.",
           "It's really very, VERY runny, sir.",
           shopkeeper="Michael Palin",
           client="John Cleese",
           sketch="Cheese Shop Sketch")

print(concat("earch","mars","venus"))
print(dir())
print(fib2(1000))


    