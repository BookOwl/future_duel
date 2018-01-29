#!/usr/bin/env python3
import os, sys, collections, random

READY    = b"\x72"
LOAD     = b"\x30"
FIRE_B   = b"\x31"
FIRE_P   = b"\x32"
SHIELD_B = b"\x33"
SHIELD_P = b"\x34"

def main():
    fp = os.fdopen(sys.stdout.fileno(), "wb")
    fp.write(READY)
    fp.flush()
    fp.write(LOAD)
    fp.flush()
    actions = [LOAD, FIRE_B, ]
    ammo = 0
    while True:
        action = actions.popleft()
        fp.write(action)
        fp.flush()
        if action == LOAD:
            ammo += 1
        elif action == FIRE_B:
            ammo -= 1
        elif action == FIRE_P:
            ammo -= 2
        chance = random.random()
        if ammo >= 2 and chance < 0.5:
            actions.append(random.choice([FIRE_B, FIRE_P]))
        elif ammo == 0:
            actions.append(LOAD)
        else:
            actions.append(random.choice([SHIELD_B, SHIELD_P]))

if __name__ == '__main__':
    main()