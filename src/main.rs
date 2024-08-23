use rand::Rng;
use textwrap::wrap;

fn main() {
    let quotelist = vec![
        "You awaken within a dream of dark skies and boiling clouds. Night on the mountaintop, a rough cloak pulled tight around you. Thunder tears the sky and a rain of hot embers pours through the rift, destroying your dream body in an instant…",
        "In your sleep, you crouch in the middle of a vast and gloomy cavern while a huge beast prowls in circles around you, dragging the darkness behind it. Emerald flames flicker at its edges and the growl of its throat lingers long into the waking day…",
        "You dream of eating roast hog at a feast. It is a simple and delicious meal. If only all dreams could be like this.",
        "Nine maidens in robes of white dance barefoot at midnight. Nine wolves hunt a white hart in the greenwood. Nine bells ring out on the mountainside and you awake.",
        "You dream of a hundred ghosts crowding thickly around your bedside, seeking warmth and life until something thumps deep below the ground and they jump like crumbs on a drumskin and are gone.",
        "You dream of running through a meadow, the sky alight with pale fire. There is a thunder of hoofbeats behind you but when you turn, nothing is there. You awaken with your heart pounding in your chest.",
        "In your dream, the forest rises before you, dragging the trees upward like a cloak, its dark mass hiding the stars. At the mountain's peak, vast antlers frame the moon…",
        "In your dream, you walk through a hall of smiling warriors and gracious maidens. You join with their mirth until you realise that you are naked and the small bronze shield you are carrying is not enough to cover your shame. You greet the morning with gratitude.",
        "Dark-eyed Loki approaches you in your dream and gifts you a ring for each hand. He tells you that as long as they stay on your fingers, you will never hear an insulting word again. You wake with your fingers in your ears.",
        "You climb a winding staircase, curled tightly within a tall tower, until you arrive at the top and look out over an endless forest. The wind blows green waves across the tree-tops and beneath the surface, dark shapes stir…",
        "You dream that you are flying over mountaintops, all of Valheim spread out below you. As you wheel and dive in the cold air, a great shape soars up past you to block the sun. In the darkness, it speaks. Seek me.",
        "You dream of a river running uphill, of green shoots turning downward into the earth…",
        "Once again, you run at the head of your warriors, the weight of your father's axe in your hand. You wake with the war-cry on your lips…",
        "In your dream, you sit beside a fire in a great hall, surrounded by the chatter of familiar voices. Their faces blur like smoke and their names slip your mind, but the warmth of their memory lingers…",
        "You stand at the prow of a leaping ship, the salt spray before you and the joyful shriek of gulls above. Folded within a dream, you remember what it was like to be alive in the land of your birth.",
        "You dream of a great tree reaching out through the night. One half of its branches crackle with flames, the others are green with leaves.",
        "Amidst the crash of arms, on the dark and glimmering plain of sleep, a face swells snarling before you. Your shield arm hangs limp, your spear is broken. You welcome the cold blade when it comes. From a dream of death, you awaken to death itself.",
        "You dream you are lying on your back in a meadow, gazing upward at the clouds. Your name is nothing, your mind is free of thought. But there is a warm hand in yours. In the dream, you are laughing. But when you awaken, you find your face damp with tears.",
        "On a boat carved from dark wood, beneath ragged sails, you lie with your arms folded across your chest. Blurred faces, like thumbprints on the darkness, croon familiar songs as they push you out to float on a sea as black and flat as glass.",
        "You lie on the battlefield, dreaming eyes turned upward to a sky veiled by smoke. The calls of your warriors grow fainter and your eyes close for a second time. Great talons slide beneath you and you feel yourself rising, lifted from your body like a babe from its crib…",
        "You fall into the deep well of sleep and dream only of darkness.",
        "You dream of a bright hall filled with gracious warriors and fair maidens. The air hums with song, the boards groan under the weight of steaming dishes, the mead flows like water. You awake slowly with the laughter still ringing in your ears…",
        "You sleep in fits and fretful dreams, the weight of the nightmare heavy on your chest. When morning comes, you greet it with relief.",
        "Sleep is a river and dreams are live fish. You wake in the morning with your net empty.",
        "You dream you are hunting with your companions, running high over green hills and down through mist-haunted valleys. Ahead of you, your prey stumbles and you leap forward, sinking your teeth into warm flesh. When you wake, the taste of metal lingers in your mouth.",
        "You dream you are walking in a snowy wood when you come upon a naked child, sitting against a tree with his eyes closed but his chest moving to breathe. As you kneel beside him, you know he has been sleeping here for many centuries, waiting for you. When you touch his shoulder, you both awaken.",
        "You dream of a mighty bear, sleeping deep below the earth in the winter of the world. It turns in its sleep, folds upon folds of flesh and fur. It has no head, no limbs. A vast mass of bear flesh, mercifully quiet.",
        "You fall asleep planning your next day's labor and in your dreams you complete it, hewing wood, foraging for food and hunting after swift deer. You return home exhausted but happy, only to awaken and find the day is still ahead of you...",
        "In a chamber hung with golden drapes, you kneel before the throne of a veiled king. Sleep is but a mask, he tells you, lifting the veil slowly. You wake screaming.",
    ];

    // seed the random number generator
    let mut rng = rand::thread_rng();

    // generate a random index within bounds
    let random_index = rng.gen_range(0..quotelist.len());

    // prepare and print string
    let formatted_string = format!("“{}”", quotelist[random_index]);
    let wrapped_string = wrap(&formatted_string, 72);
    for line in &wrapped_string {
        println!("{}", line);
    }
}
