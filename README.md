# player-editor

Edit Bedrock player data with the CLI! (kind of)

This tool lets you read and write player data NBT from a Bedrock world's LevelDB database.

## Preface

**Please ensure to backup your world before editing it with this tool. I have battle-tested that no issues should occur, but you never know, as this is making changes to the database itself, so please always take the safe route. Thank you!**

## Usage

First, you run the command to extract the files, which will dump their contents as NBT files inside of a temporary `_player` folder inside the folder of the world.

```sh
./player-editor <path-to-your-world> --read
```

After this runs, you can then open these with an editor of your choice, [Dovetail](https://offroaders123.github.io/Dovetail/) will work for sure, [there are a few others](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT#Utilities) that support Bedrock little-endian files you can use as well.

Now that you've edited your player data NBT, you run the command again, but this time in write mode, which will add these new updated contents back into the world's database.

```sh
./player-editor <path-to-your-world> --write
```

Optionally, now you can remove the temporary `_player` folder that this command made. Now you can move this world folder back into your `games/com.mojang/minecraftWorlds` folder, and the changes should now be visible in-game!

Currently this tool only focuses on editing player-related data, I plan to make a more extensive concept similar to this, but built around a GUI, and for all features within the world database. You can check it out over at [Offroaders123/world_nbt](https://github.com/Offroaders123/world_nbt).
