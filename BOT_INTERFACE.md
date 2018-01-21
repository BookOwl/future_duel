Bots interface with the game runner by sending and reading bytes from stdin/stdout.

The following commands are available:

|    Command     |  Byte  | Action |
| -------------- | ------ | ------ |
| Ready          | `0x72` | Notifies the game runner that your bot is ready to start. Must be the first thing sent to stdout.
| Load ammo      | `0x30` | Takes 100ms and awards one ammo
| Fire bullet    | `0x31` | Takes 1 ammo and fires a bullet at the opponent. If you didn't have any ammo when you fired you die. If the opponent didn't have a metal shield up when you fired it dies.
| Fire plasma    | `0x32` | Takes 2 ammo and fires a bullet at the opponent. If you didn't have 2 or more ammo when you fired you die. If the opponent didn't have a plasma shield up when you fired it dies.
| Metal shield   | `0x33` | Raises a shield that protects against bullets, but not plasma.
| Thermal shield | `0x34` | Raise a shield that protects against plasma, but not bullets.

You tell the game runner what action you want to perform by outputting the corresponding byte to your stdout. The game runner notifies you of your opponents move by writing the corresponding byte to your stdin.
