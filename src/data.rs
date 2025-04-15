use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "ASUS Prime Radeon RX 9070 OC 16GB GDDR6 Video Card".to_string(),
            price: 949.99,
            description: "Boost your CPU's power with the ASUS Prime Radeon RX 9070 OC video card. Equipped with advanced axial-tech fans and a phase-change GPU thermal pad, it offers optimal cooling and performance".to_string(),
            image: "/asus_graphics.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Blutron EB570 Electric Commuter Bike".to_string(),
            price: 1099.99,
            description: "Experience smooth and swift city rides with Blutron EB570 electric bike. Its 350W motor powers you to speeds of up to 32km/h, making commutes quick and efficient.".to_string(),
            image: "/blutron_ebike.jpg".to_string()
        },
        Product {
            id: 3,
            name: "bObsweep UltraVision WiFi Connected Self-Empty Robot Vacuum & Mop".to_string(),
            price: 12.99,
            description: "Keep your house clean and spotless with the bObsweep UltraVision robot vacuum and mop. With a powerful 1,200W self-emptying station, 8,000Pa suction, and Low-height Object-aware Technology, it helps you tackle dirt and debris effortlessly.".to_string(),
            image: "/bobsweep_smart_mop.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Amazon Echo Show 21 Smart Display with Alexa".to_string(),
            price: 549.99,
            description: "Elevate your kitchen experience with the Amazon Echo Show 21 Smart display. Its 21-inch Full HD screen delivers crystal-clear visuals, while built-in Fire TV and Alexa provide seamless entertainment and smart home control.".to_string(),
            image: "/echo_show.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Fujifilm Instax WIDE Evo Hybrid Instant Camera".to_string(),
            price: 479.99,
            description: "Create without limits with the Fujifilm Instax WIDE Evo camera. This instant camera combines analog charm with digital precision for a photo-taking experience like no other.".to_string(),
            image: "/fujifilm_instax.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Garmin Instinct 3 Solar 45mm Bluetooth Multisport Smartwatch".to_string(),
            price: 579.99,
            description: "Stay active and connected with this Garmin Instinct 3 Solar multisport smartwatch. It helps track your steps and distance covered, and even monitors your heart rate".to_string(),
            image: "/garmin_instinct.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Apple MacBook Air 13.6 w/ Touch ID (2025)".to_string(),
            price: 1399.99,
            description: "The 13-inch MacBook Air with the M4 chip lets you fly through work and play. With Apple Intelligence, a brilliant Liquid Retina display, up to 18 hours of battery life, and a strikingly thin and light design, it’s built to last and can take on just about anything, anywhere.".to_string(),
            image: "/macbook_air.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Google Nest Wi-Fi Smart Learning Thermostat".to_string(),
            price: 329.99,
            description: "Meet the Google Nest Smart Learning thermostat. This 4th generation thermostat is a beautiful, brilliant way to help save energy and keep your home comfortable.".to_string(),
            image: "/nest_thermostat.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Ninja CREAMi Swirl 0.47L Ice Cream Maker".to_string(),
            price: 429.99,
            description: "Satisfy your sweet tooth with the Ninja CREAMi 0.47L ice cream maker. This smart maker lets you easily create everything from fruity sorbets to creamy soft serve.".to_string(),
            image: "/ninja_creami.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Nintendo Sound Clock".to_string(),
            price: 129.99,
            description: "Wake up to fun, game-inspired alarm with the Nintendo Sound Clock: Alarmo. Choose from 35 scenes across iconic Nintendo titles to spice up your morning routine. ".to_string(),
            image: "/nintendo_sound_clock.jpg".to_string()
        },
        Product {
            id: 11,
            name: "Nintendo Switch (OLED Model) Super Mario Bros".to_string(),
            price: 449.99,
            description: "Play anytime, anywhere, with the Nintendo Switch (OLED Model) Super Mario Bros. Wonder bundle. Team up with three friends locally. Or partner up with other players around the world using Nintendo Switch Online membership1 (included).".to_string(),
            image: "/nintendo_switch.jpg".to_string()
        },
        Product {
            id: 12,
            name: "NIU KQi2 Pro Electric Scooter".to_string(),
            price: 849.99,
            description: "Commute smarter with the NIU KQi2 Pro electric scooter. This e-Bike delivers speeds of up to 28km/h and a generous 40km range, perfect for daily rides.".to_string(),
            image: "/niu_electric_scooter.jpg".to_string()
        },
        Product {
            id: 13,
            name: "Philips 3000 Series Retrofit Wi-Fi Smart Lock".to_string(),
            price: 159.99,
            description: "Control and monitor your doors from anywhere with this Philips retrofit lock. Easy to install, it works with your existing keys and offers smart control through Wi-Fi.".to_string(),
            image: "/phillips_smart_lock.jpg".to_string()
        },
        Product {
            id: 14,
            name: "PlayStation 5 Slim Digital Edition Console".to_string(),
            price: 579.99,
            description: "Take your gaming to the next level with PlayStation 5 Slim Digital Edition console. It features a redesigned PS5 with a slimmer profile that looks great and takes up less space.".to_string(),
            image: "/playstation_5_slim.jpg".to_string()
        },
        Product {
            id: 15,
            name: "EF EcoFlow Portable Power Station RIVER 3+, 286Wh".to_string(),
            price: 485.99,
            description: "The EF ECOFLOW RIVER 3 Plus portable power station delivers 600W continuous power and 1200W surge power, with 3 AC outlets and a LiFePO4 battery. It features 10 ms UPS protection, expandable capacity, fast recharging in 1 hour, and solar charging in 1.5 hours.".to_string(),
            image: "/portable_power_station.png".to_string()
        },
        Product {
            id: 16,
            name: "Samsung 65 inch 4K UHD HDR LED Tizen OS Smart TV".to_string(),
            price: 499.99,
            description: "Upgrade your immersive entertainment experience with this Samsung 65-inch Smart TV. Featuring 4K Ultra HD resolution and 60Hz refresh rate, this TV delivers crisp and clear picture quality with smoother visuals.".to_string(),
            image: "/samsung_65_tv.jpg".to_string()
        },
        Product {
            id: 17,
            name: "Starlink Mini Kit".to_string(),
            price: 549.99,
            description: "Stay connected anywhere, anytime with the Starlink mini kit. Engineered by SpaceX, this all-in-one kit features Wi-Fi 5 and an Ethernet port for connecting third-party routers and mesh.".to_string(),
            image: "/starlink_mini.jpg".to_string()
        },
        Product {
            id: 18,
            name: "Incase Designed by Microsoft Bluetooth Optical Mouse - Blackt".to_string(),
            price: 24.99,
            description: "The Incase Bluetooth optical mouse offers a no-fuss solution for everyday computer use. It features precise optical tracking with a maximum of 1,000dpi to ensure accurate cursor movement.".to_string(),
            image: "/wireless_mouse.jpg".to_string()
        }
    ]
}