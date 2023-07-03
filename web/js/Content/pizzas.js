export const Pizzatypes = {
    normal: "normal",
    spicy: "spicy",
    veggie: "veggie",
    fungi: "fungi",
    chill: "chill",
}

export const Pizzas = {
    "s001": {
        name: "Slice Samurai",
        type: Pizzatypes.spicy,
        description: "simple starter pizza",
        src: "images/characters/pizzas/s001.png",
        icon: "images/icons/spicy.png",
        actions: ["damage1"],
        xp_adder: 10,
    },
    "s002": {
        name: "Bacon Brigade",
        description: "spicy level 2",
        type: Pizzatypes.spicy,
        src: "images/characters/pizzas/s002.png",
        icon: "images/icons/spicy.png",
        actions: ["damage1", "saucyStatus"],
        xp_adder: 15,
    },
    "s003": {
        name: "Chilli Extreme",
        description: "spicy level 3",
        type: Pizzatypes.spicy,
        src: "images/characters/pizzas/s003.png",
        icon: "images/icons/spicy.png",
        actions: ["damage1", "saucyStatus", "clumsyStatus"],
        xp_adder: 20,
    },
    "v001": {
        name: "Call me Kale",
        type: Pizzatypes.veggie,
        src: "images/characters/pizzas/v001.png",
        icon: "images/icons/veggie.png",
        actions: ["damage2"],
        xp_adder: 30,
    },
    "v002": {
        name: "Broccoli blaster",
        type: Pizzatypes.veggie,
        src: "images/characters/pizzas/v002.png",
        icon: "images/icons/veggie.png",
        actions: ["damage2", "saucyStatus", "reboundStatus"],
        xp_adder: 40,
    },
    "f001": {
        name: "Portobello Express",
        type: Pizzatypes.spicy,
        src: "images/characters/pizzas/f001.png",
        icon: "images/icons/fungi.png",
        actions: ["damage1"],
    },
}