extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


/// User type
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
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
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
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
pub struct Chat {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    type: String,
    offset: u32,
    length: u32,
    #[serde(default)]
    url: String,
    #[serde(default)]
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    file_id: String,
    width: u32,
    height: u32,
    #[serde(default)]
    file_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    file_id: String,
    duration: u32,
    #[serde(default)]
    mime_type: String,
    #[serde(default)]
    file_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {
    file_id: String,
    length: u32,
    duration: u32,
    #[serde(default)]
    thumb: PhotoSize,
    #[serde(default)]
    file_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    user_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    longitude: f32,
    latitude: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    #[serde(default)]
    foursquare_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    total_count: u32,
    photos: Vec<Vec<PhotoSize>>,
}

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link
/// `https://api.telegram.org/file/bot<token>/<file_path>.
/// It is guranteed that the link will be valid for at least
/// 1 hour. When the link expires, a new once can be requested
/// by calling `getFile`.
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    file_id: String,
    #[serde(default)]
    file_size: u32,
    #[serde(default)]
    file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(default)]
    resize_keyboard: bool,
    #[serde(default)]
    one_time_keyboard: bool,
    #[serde(default)]
    selective: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    text: String,
    #[serde(default)]
    request_contact: bool,
    #[serde(default)]
    request_location: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(default)]
    selective: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    id: String,
    from: User,
    #[serde(default)]
    message: Message,
    #[serde(default)]
    inline_message_id: String,
    #[serde(default)]
    chat_instance: String,
    #[serde(default)]
    data: String,
    #[serde(default)]
    game_short_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForceReply {
    force_reply: bool,
    #[serde(default)]
    selective: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMember {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    #[serde(default)]
    migrate_to_chat_id: u32,
    #[serde(default)]
    retry_after: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaPhoto {
    type: String,
    media: String,
    #[serde(default)]
    caption: String,
    #[serde(default)]
    parse_mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaVideo {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct InputFile {
}