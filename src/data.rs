use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Beats By Dr. Dre Studio Pro Over-Ear Noise Cancelling Bluetooth Headphones".to_string(),
            price: 469.99,
            description: "Beats By Dr. Dre Studio Pro over-ear headphones with Active Noise Cancelling blocks unwanted external noise, while Transparency Mode lets outside sound in".to_string(),
            image: "/beats.jpg".to_string()
        },
        Product {
            id: 2,
            name: "ASUS ROG Swift 26.5' QHD 240Hz 0.03ms GTG OLED G-Sync Gaming Monitor".to_string(),
            price: 1099.99,
            description: "Bring your games to life with the ASUS ROG Swift 26.5' gaming monitor. It features a 26.5' OLED display with QHD resolution and 240Hz refresh rate that keep your gaming action detailed and crisp".to_string(),
            image: "/asus.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Blue Microphones Yeti USB Microphone - Silver".to_string(),
            price: 129.99,
            description: "Transform the audio of your recordings with the Yeti USB microphone. It produces a clear, broadcast-quality sound for music, podcasts, and more.".to_string(),
            image: "/blueyeti.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Apple iPhone 13 128GB - Midnight".to_string(),
            price: 699.99,
            description: "Stand out and show-off your superpowers with the iPhone 13. It features the powerful A15 Bionic chip, superfast 5G to download and stream high-quality video, a bright 6.1' Super Retina XDR display, and Ceramic Shield for better drop performance.".to_string(),
            image: "/iphone13.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Apple - 10.9-Inch iPad (10th Generation)".to_string(),
            price: 329.99,
            description: "A tablet with 64GB storage and Wi-Fi connectivity, suitable for multimedia, productivity, and creative tasks.".to_string(),
            image: "/ipad.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Apple - 13-inch MacBook Air with M2 Chip".to_string(),
            price: 999.99,
            description: "Features a new design, 16GB memory, 256GB SSD, and the Apple M2 chip for enhanced performance. Perfect for productivity and portability.".to_string(),
            image: "/macbook.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Logitech G502 LIGHTSPEED 25600 DPI Wireless Optical Gaming Mouse - Black".to_string(),
            price: 149.99,
            description: "Play better with the Logitech G502 LIGHTSPEED gaming mouse. It brings you superfast LIGHTSPEED wireless, a 1ms response, and POWERPLAY compatibility. Eleven programmable buttons, 16.8 million colours, and 6 adjustable, configurable weights make the mouse uniquely yours.".to_string(),
            image: "/logitech.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Razer Huntsman Mini Backlit Optical Gaming Keyboard - White".to_string(),
            price: 109.99,
            description: "Speed is crucial when it comes to gameplay and this Razer Huntsman Mini backlit optical gaming keyboard provides fast, light, and smooth actuations to enhance the way you play. ".to_string(),
            image: "/razerhuntsman.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Samsung 85' 4K UHD HDR QLED Tizen OS Smart TV".to_string(),
            price: 2399.99,
            description: "Enjoy the next level of visual immersion with this Samsung Smart TV. Featuring 4K Ultra HD resolution and 4K AI Upscaling, you can expect impressive picture quality every time.".to_string(),
            image: "/samsungtv.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Xbox Series S 512GB Console".to_string(),
            price: 379.99,
            description: "Combining speed and performance, the Xbox Series S console lets you play games your way.".to_string(),
            image: "/xbox.jpg".to_string()
        }
    ]
}