#!/usr/bin/python3
# -*- coding:utf-8 -*-

import numpy as np

def run():
    tensor_0D()
    tensor_1D()
    tensor_2D()
    tensor_3D()
    

def tensor_0D():
    '''
    '''
    x = np.array(12)
    print("0D x.ndim = ", x.ndim)
    
    
def tensor_1D():
    x = np.array([12, 13, 14, 15])
    print("1D x.ndim = ", x.ndim)
    
def tensor_2D():
    x = np.array(
        [
            [12, 13, 14, 15],
            [0,34,39,0]
        ]
    )
    print("2D x.ndim = ", x.ndim)
    
def tensor_3D():
    x = np.array(
        [
            [
                [12, 13, 14, 15],
                [0,34,39,0]
            ],
            [
                [12, 13, 14, 15],
                [0,34,39,0]
            ]
        ]
    )
    print("3D x.ndim = ", x.ndim)
    