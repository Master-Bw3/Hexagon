    , { signature = "qaawdde", internalName = "swizzle", action = noAction, displayName = "", outputOptions = [], selectedOutput = Nothing, startDirection = East } -- do this
    , { signature = "eqqq", internalName = "random", action = spellNoInput, displayName = "Entropy Reflection", outputOptions = [NumberType], selectedOutput = Just (NumberType, Number 0), startDirection = East }
    , { signature = "de", internalName = "print", action = print, displayName = "Reveal", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "aawaawaa", internalName = "explode", action = explode, displayName = "Explosion", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "ddwddwdd", internalName = "explode/fire", action = explodeFire, displayName = "Fireball", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "awqqqwaqw", internalName = "add_motion", action = addMotion, displayName = "Impulse", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "awqqqwaq", internalName = "blink", action = blink, displayName = "Blink", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qaqqqqq", internalName = "break_block", action = breakBlock, displayName = "Break Block", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "eeeeede", internalName = "place_block", action = placeBlock, displayName = "Place Block", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "awddwqawqwawq", internalName = "colorize", action = colorize, displayName = "Internalize Pigment", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "aqawqadaq", internalName = "create_water", action = createWater, displayName = "Create Water", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "dedwedade", internalName = "destroy_water", action = destroyWater, displayName = "Destroy Liquid", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "aaqawawa", internalName = "ignite", action = ignite, displayName = "Ignite Block", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "ddedwdwd", internalName = "extinguish", action = extinguish, displayName = "Extinguish Area", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqa", internalName = "conjure_block", action = conjureBlock, displayName = "Conjure Block", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqd", internalName = "conjure_light", action = conjureLight, displayName = "Conjure Light", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wqaqwawqaqw", internalName = "bonemeal", action = bonemeal, displayName = "Overgrow", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqwaeaeaeaeaea", internalName = "recharge", action = recharge, displayName = "Recharge Item", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qdqawwaww", internalName = "erase", action = erase, displayName = "Erase Item", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wqaqwd", internalName = "edify", action = edify, displayName = "Edify Sapling", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "adaa", internalName = "beep", action = beep, displayName = "Make Note", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waqqqqq", internalName = "craft/cypher", action = craftArtifact Cypher, displayName = "Craft Cypher", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wwaqqqqqeaqeaeqqqeaeq", internalName = "craft/trinket", action = craftArtifact Trinket, displayName = "Craft Trinket", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wwaqqqqqeawqwqwqwqwqwwqqeadaeqqeqqeadaeqq", internalName = "craft/artifact", action = craftArtifact Artifact, displayName = "Craft Artifact", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqaqwawaw", internalName = "potion/weakness", action = potion, displayName = "White Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqawwawawd", internalName = "potion/levitation", action = potionFixedPotency, displayName = "Blue Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqaewawawe", internalName = "potion/wither", action = potion, displayName = "Black Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqadwawaww", internalName = "potion/poison", action = potion, displayName = "Red Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqqqadwawaw", internalName = "potion/slowness", action = potion, displayName = "Green Sun's Nadir", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waeawae", internalName = "sentinel/create", action = sentinelCreate, displayName = "Summon Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qdwdqdw", internalName = "sentinel/destroy", action = sentinelDestroy, displayName = "Banish Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waeawaede", internalName = "sentinel/get_pos", action = sentinelGetPos, displayName = "Locate Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "waeawaedwa", internalName = "sentinel/wayfind", action = sentinelWayfind, displayName = "Wayfind Sentinel", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "qqqwqqqqqaq", internalName = "akashic/read", action = akashicRead, displayName = "Akasha's Distillation", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "eeeweeeeede", internalName = "akashic/write", action = akashicWrite, displayName = "Akasha's Gambit", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "aqqqqq", internalName = "read", action = read, displayName = "Scribe's Reflection", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wawqwqwqwqwqw", internalName = "read/entity", action = readChronical, displayName = "Chronicler's Purification", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "deeeee", internalName = "write", action = write, displayName = "Scribe's Gambit", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wdwewewewewew", internalName = "write/entity", action = writeChronical, displayName = "Chronicler's Gambit", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "aqqqqqe", internalName = "readable", action = readable, displayName = "Auditor's Reflection", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wawqwqwqwqwqwew", internalName = "readable/entity", action = makeConstant (Boolean False), displayName = "Auditor's Purification", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "deeeeeq", internalName = "writable", action = writable, displayName = "Assessor's Reflection", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "wdwewewewewewqw", internalName = "writable/entity", action = makeConstant (Boolean False), displayName = "Assessor's Purification", outputOptions = [], selectedOutput = Nothing, startDirection = East }
    , { signature = "awaawa", internalName = "save_macro", action = saveMacro, displayName = "Save Macro", outputOptions = [], selectedOutput = Nothing, startDirection = Southeast }