extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Read;

use serde_json::Value;


fn main() {
    let bot_token = "547846290:AAFwQk6rAO7_77Yn1aEF5CLojLoIP12jhYs";
    println!("My bot token: {}", bot_token);
    println!("Booting bot..");
    let bot = Bot::boot(&bot_token);
    let me = bot.get_me();
    println!("{:?}", me.unwrap());
}

/// User type
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    is_bot: bool,
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    username: String,
    #[serde(default)]
    language_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message_id: u32,
    #[serde(default)]
    from: User,
    date: u32,
    chat: Chat,
    #[serde(default)]
    forward_from: User,
    #[serde(default)]
    forward_from_chat: Chat,
    #[serde(default)]
    forward_from_message_id: u32,
    #[serde(default)]
    forward_signature: String,
    #[serde(default)]
    forward_date: u32,
    #[serde(default)]
    reply_to_message: Message,
    #[serde(default)]
    edit_date: u32,
    #[serde(default)]
    media_group_id: String,
    #[serde(default)]
    author_signature: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    entities: Vec<MessageEntity>,
    #[serde(default)]
    caption_entities: Vec<MessageEntity>,
    #[serde(default)]
    audio: Audio,
    #[serde(default)]
    document: Document,
    #[serde(default)]
    game: Game,
    #[serde(default)]
    photo: Vec<PhotoSize>,
    #[serde(default)]
    sticker: Sticker,
    #[serde(default)]
    video: Video,
    #[serde(default)]
    voice: Voice,
    #[serde(default)]
    video_note: VideoNote,
    #[serde(default)]
    caption: String,
    #[serde(default)]
    contact: Contact,
    #[serde(default)]
    location: Location,
    #[serde(default)]
    venue: Venue,
    #[serde(default)]
    new_chat_members: Vec<User>,
    #[serde(default)]
    left_chat_member: User,
    #[serde(default)]
    new_chat_photo: Vec<PhotoSize>,
    #[serde(default)]
    delete_chat_photo: bool,
    #[serde(default)]
    group_chat_created: bool,
    #[serde(default)]
    supergroup_chat_created: bool,
    #[serde(default)]
    channel_chat_created: bool,
    #[serde(default)]
    migrate_to_chat_id: u32,
    #[serde(default)]
    migrate_from_chat_id: u32,
    #[serde(default)]
    pinned_message: Message,
    #[serde(default)]
    invoice: Invoice,
    #[serde(default)]
    successful_payment: SuccessfulPayment,
    #[serde(default)]
    connected_website: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Chat {
    id: u32,
    type: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    username: String,
    #[serde(default)]
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    all_members_are_adminstrators: bool,
    #[serde(default)]
    photo: ChatPhoto,
    #[serde(default)]
    description: String,
    #[serde(default)]
    invite_link: String,
    #[serde(default)]
    pinned_message: Message,
    #[serde(default)]
    sticker_set_name: String,
    #[serde(default)]
    can_set_sticker_set: bool,
}

struct MessageEntity {
    type: String,
    offset: u32,
    length: u32,
    #[serde(default)]
    url: String,
    #[serde(default)]
    user: User,
}

struct PhotoSize {
    file_id: String,
    width: u32,
    height: u32,
    #[serde(default)]
    file_size: u32,
}

struct Audio {
    file_id: String,
    duration: u32,
    #[serde(default)]
    performer: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    mime_type: String,
    #[serde(default)]
    file_size: u32,
}

struct Document {
    file_id: String,
    #[serde(default)]
    thumb: PhotoSize,
    #[serde(default)]
    file_name: String,
    #[serde(default)]
    mime_type: String,
    #[serde(default)]
    file_size: u32,
}

struct Video {
    file_id: String,
    width: u32,
    height: u32,
    duration: u32,
    #[serde(default)]
    thumb: PhotoSize,
    #[serde(default)]
    mime_type: String,
    #[serde(default)]
    file_size: u32,
}

struct Voice {
    file_id: String,
    duration: u32,
    #[serde(default)]
    mime_type: String,
    #[serde(default)]
    file_size: u32,
}

struct VideoNote {
    file_id: String,
    length: u32,
    duration: u32,
    #[serde(default)]
    thumb: PhotoSize,
    #[serde(default)]
    file_size: u32,
}

struct Contact {
    phone_number: String,
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    user_id: u32,
}

struct Location {
    longitude: f32,
    latitude: f32,
}

struct Venue {
    location: Location,
    title: String,
    address: String,
    #[serde(default)]
    foursquare_id: String,
}

struct UserProfilePhotos {
    total_count: u32,
    photos: Vec<Vec<PhotoSize>>,
}

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link
/// `https://api.telegram.org/file/bot<token>/<file_path>.
/// It is guranteed that the link will be valid for at least
/// 1 hour. When the link expires, a new once can be requested
/// by calling `getFile`.
struct File {
    file_id: String,
    #[serde(default)]
    file_size: u32,
    #[serde(default)]
    file_path: String,
}

struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(default)]
    resize_keyboard: bool,
    #[serde(default)]
    one_time_keyboard: bool,
    #[serde(default)]
    selective: bool,
}

struct KeyboardButton {
    text: String,
    #[serde(default)]
    request_contact: bool,
    #[serde(default)]
    request_location: bool,
}

struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(default)]
    selective: bool,
}

struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

struct InlineKeyboardButton {
    text: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    callback_data: String,
    #[serde(default)]
    switch_inline_query: String,
    #[serde(default)]
    switch_inline_query_current_chat: String,
    #[serde(default)]
    callback_game: CallbackGame,
    #[serde(default)]
    pay: bool,
}

struct CallbackQuery {
    id: String,
    from: User,
    #[serde(default)]
    message: Message,
    #[serde(default)]
    inline_message_id: String,
    #[serde(default)]
    chat_instance: String,
    #[serde(derive)]
    data: String,
    #[serde(default)]
    game_short_name: String,
}

struct ForceReply {
    force_reply: bool,
    #[serde(default)]
    selective: bool,
}

struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

struct ChatMember {
    user: User,
    status: String,
    #[serde(default)]
    until_date: u32,
    #[serde(default)]
    can_be_edited: bool,
    #[serde(default)]
    can_change_info: bool,
    #[serde(default)]
    can_post_messages: bool,
    #[serde(default)]
    can_edit_messages: bool,
    #[serde(default)]
    can_delete_messages: bool,
    #[serde(default)]
    can_invite_users: bool,
    #[serde(default)]
    can_restrict_members: bool,
    #[serde(default)]
    can_pin_messages: bool,
    #[serde(default)]
    can_promote_members: bool,
    #[serde(default)]
    can_send_messages: bool,
    #[serde(default)]
    can_send_media_messages: bool,
    #[serde(default)]
    can_send_other_messages: bool,
    #[serde(default)]
    can_add_web_pae_previews: bool,
}

struct ResponseParameters {
    #[serde(default)]
    migrate_to_chat_id: u32,
    #[serde(default)]
    retry_after: u32,
}

struct InputMediaPhoto {
    type: String,
    media: String,
    #[serde(default)]
    caption: String,
    #[serde(default)]
    parse_mode: String,
}

struct InputMediaVideo {
    type: String,
    media: String,
    #[serde(default)]
    caption: String,
    #[serde(default)]
    parse_mode: String,
    #[serde(default)]
    width: u32,
    #[serde(default)]
    height: u32,
    #[serde(default)]
    duration: u32,
    #[serde(default)]
    supports_streaming: bool,
}

struct InputFile {
}











struct Bot {
    token: String,
    reqwest_client: reqwest::Client,
}

impl Bot {
    fn boot(api_token: &str) -> Bot {
        Bot {
            token: api_token.to_string(),
            reqwest_client: reqwest::Client::new(),
        }
    }

    fn form_method(&self, method: &str) -> String {
        let req = format!("https://api.telegram.org/bot{}/{}", self.token, method);
        req
    }

    fn handle_response(res: reqwest::Response) -> Result<reqwest::Response, &'static str> {
        if res.status().is_success() {
            Ok(res)
        } else if res.status().is_server_error() {
            Err("Server error!")
        } else {
            Err("Something went wrong!")
        }
    }

    fn get_me(&self) -> Result<User, &'static str> {
        let req = Bot::form_method(&self, "getMe"); 
        let res = self.reqwest_client.get(&req).send().unwrap();
        match Bot::handle_response(res) {
            Ok(mut res) => {
                let mut content = String::new();
                res.read_to_string(&mut content).unwrap();
                let value: Value = serde_json::from_str(&content).unwrap();
                let me: User = serde_json::from_str(&value["result"].to_string()).unwrap();
                Ok(me)
            },
            Err(e) => Err(e),
        }
    }


}
