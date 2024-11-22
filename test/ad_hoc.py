import hok_io

class Vec():
    x = 0
    y = 0

    def __init__(a,b):
        x = a
        y = b

def move(pos):
    return Vec(50 - pos.x,50 - pos.y)



hok_io.ad_hok(move)