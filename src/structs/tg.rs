use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    last_name: Option<String>,
    pub username: Option<String>,
    language_code: Option<String>,
    is_premium: Option<bool>,
    added_to_attachment_menu: Option<bool>,
    can_join_groups: Option<bool>,
    can_read_all_group_messages: Option<bool>,
    supports_inline_queries: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ChatPhoto {
    small_file_id: String,
    small_file_unique_id: String,
    big_file_id: String,
    big_file_unique_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatPermissions {
    can_send_messages: Option<bool>,
    can_send_audios: Option<bool>,
    can_send_documents: Option<bool>,
    can_send_photos: Option<bool>,
    can_send_videos: Option<bool>,
    can_send_video_notes: Option<bool>,
    can_send_voice_notes: Option<bool>,
    can_send_polls: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
    can_manage_topics: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    longitude: f64,
    latitude: f64,
    horizontal_accuracy: Option<f64>,
    live_period: Option<u64>,
    heading: Option<u64>,
    proximity_alert_radius: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ChatLocation {
    location: Location,
    address: String
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: u64,
    r#type: String,
    title: Option<String>,
    pub username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    is_forum: Option<bool>,
    photo: Option<ChatPhoto>,
    active_usernames: Option<Vec<String>>,
    emoji_status_custom_emoji_id: Option<String>,
    bio: Option<String>,
    has_private_forwards: Option<String>,
    has_restricted_voice_and_video_messages: Option<String>,
    join_to_send_messages: Option<String>,
    join_by_request: Option<String>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Box<Message>>,
    permissions: Option<ChatPermissions>,
    slow_mode_delay: Option<u64>,
    message_auto_delete_time: Option<u64>,
    has_aggressive_anti_spam_enabled: Option<bool>,
    has_hidden_members: Option<bool>,
    has_protected_content: Option<bool>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
    linked_chat_id: Option<i64>,
    location: Option<ChatLocation>,
}

#[derive(Deserialize, Debug)]
pub struct MessageEntity {
    r#type: String,
    offset: u64,
    length: u64,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
    custom_emoji_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: u64,
    height: u64,
    file_size: Option<u64>
}

#[derive(Deserialize, Debug)]
pub struct Animation {
    file_id: String,
    file_unique_id: String,
    width: u64,
    height: u64,
    duration: u64,
    thumbnail: Option<PhotoSize>,
    mime_type: Option<String>,
    file_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Audio {
    file_id: String,
    file_unique_id: String,
    duration: u64,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u64>,
    thumbnail: Option<PhotoSize>
}

#[derive(Deserialize, Debug)]
pub struct Document {
    file_id: String,
    file_unique_id: String,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Video {
    file_id: String,
    file_unique_id: String,
    width: u64,
    height: u64,
    duration: u64,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: u64,
    duration: u64,
    thumbnail: Option<PhotoSize>,
    file_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    file_id: String,
    file_unique_id: String,
    file_size: Option<u64>,
    file_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

#[derive(Deserialize, Debug)]
pub struct Sticker {
    file_id: String,
    file_unique_id: String,
    r#type: String,
    width: u64,
    height: u64,
    is_animated: bool,
    is_video: bool,
    thumbnail: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    premium_animation: Option<File>,
    mask_position: Option<MaskPosition>,
    custom_emoji_id: Option<String>,
    needs_repainting: Option<bool>,
    file_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: u64,
    mime_type: Option<String>,
    file_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<i64>,
    vcard: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Dice {
    emoji: String,
    value: u8,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}

#[derive(Deserialize, Debug)]
pub struct PollOption {
    text: String,
    voter_count: u64,
}

#[derive(Deserialize, Debug)]
pub struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    total_voter_count: u64,
    is_closed: bool,
    is_anonymous: bool,
    r#type: String,
    allows_multiple_answers: bool,
    correct_option_id: Option<u64>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<u64>,
    close_date: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: u64,
}

#[derive(Deserialize, Debug)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: u64,
}

#[derive(Deserialize, Debug)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

#[derive(Deserialize, Debug)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: u64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id:	String,
}

#[derive(Deserialize, Debug)]
pub struct UserShared {
    request_id: i64,
    user_id: i64,
}

#[derive(Deserialize, Debug)]
pub struct ChatShared {
    request_id: i64,
    chat_id: i64,
}

#[derive(Deserialize, Debug)]
pub struct WriteAccessAllowed {
    web_app_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PassportFile {
    file_id: String,
    file_unique_id: String,
    file_size: u64,
    file_date: u64,
}

#[derive(Deserialize, Debug)]
pub struct EncryptedPassportElement {
    r#type: String,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: Option<Vec<PassportFile>>,
    hash: String,
}

#[derive(Deserialize, Debug)]
pub struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}

#[derive(Deserialize, Debug)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}

#[derive(Deserialize, Debug)]
pub struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: u64,
}

#[derive(Deserialize, Debug)]
pub struct ForumTopicCreated {
    name: String,
    icon_color: u64,
    icon_custom_emoji_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ForumTopicEdited {
    name: Option<String>,
    icon_custom_emoji_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ForumTopicClosed {}

#[derive(Deserialize, Debug)]
pub struct ForumTopicReopened {}

#[derive(Deserialize, Debug)]
pub struct GeneralForumTopicHidden {}

#[derive(Deserialize, Debug)]
pub struct GeneralForumTopicUnhidden {}

#[derive(Deserialize, Debug)]
pub struct VideoChatScheduled {
    start_date: u64,
}

#[derive(Deserialize, Debug)]
pub struct VideoChatStarted {}

#[derive(Deserialize, Debug)]
pub struct VideoChatEnded {
    duration: u64,
}

#[derive(Deserialize, Debug)]
pub struct VideoChatParticipantsInvited {
    users: Vec<User>,
}

#[derive(Deserialize, Debug)]
pub struct WebAppData {
    data: String,
    button_text: String,
}

#[derive(Deserialize, Debug)]
pub struct WebAppInfo {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUrl {
    url: String,
    forward_text: Option<String>,
    bot_username: Option<String>,
    request_write_access: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct SwitchInlineQueryChosenChat {
    query: Option<String>,
    allow_user_chats: Option<bool>,
    allow_bot_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct CallbackGame {}

#[derive(Deserialize, Debug)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    web_app: Option<WebAppInfo>,
    login_url: Option<LoginUrl>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<InlineKeyboardButton>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: u64,
    message_thread_id: Option<u64>,
    pub from: Option<User>,
    sender_chat: Option<Box<Chat>>,
    pub date: u64,
    pub chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<i64>,
    forward_signature: Option<String>,
    forward_sender_name: Option<String>,
    forward_date: Option<u64>,
    is_topic_message: Option<bool>,
    is_automatic_forward: Option<bool>,
    reply_to_message: Option<Box<Message>>,
    via_bot: Option<User>,
    edit_date: Option<u64>,
    has_protected_content: Option<bool>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    pub text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
    audio: Option<Audio>,
    document: Option<Document>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    video: Option<Video>,
    video_note: Option<VideoNote>,
    voice: Option<Voice>,
    caption: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    has_media_spoiler: Option<bool>,
    contact: Option<Contact>,
    dice: Option<Dice>,
    game: Option<Game>,
    poll: Option<Poll>,
    venue: Option<Venue>,
    location: Option<Location>,
    new_chat_members: Option<Vec<User>>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    migrate_to_chat_id: Option<i64>,
    migrate_from_chat_id: Option<i64>,
    pinned_message: Option<Box<Message>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    user_shared: Option<UserShared>,
    chat_shared: Option<ChatShared>,
    connected_website: Option<String>,
    write_access_allowed: Option<WriteAccessAllowed>,
    passport_data: Option<PassportData>,
    proximity_alert_triggered: Option<ProximityAlertTriggered>,
    forum_topic_created: Option<ForumTopicCreated>,
    forum_topic_edited: Option<ForumTopicEdited>,
    forum_topic_closed: Option<ForumTopicClosed>,
    forum_topic_reopened: Option<ForumTopicReopened>,
    general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    video_chat_scheduled: Option<VideoChatScheduled>,
    video_chat_started: Option<VideoChatStarted>,
    video_chat_ended: Option<VideoChatEnded>,
    video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    web_app_data: Option<WebAppData>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Deserialize, Debug)]
pub struct InlineQuery {
    id: String,
    from: User,
    query: String,
    offset: String,
    chat_type: Option<String>,
    location: Option<Location>,
}

#[derive(Deserialize, Debug)]
pub struct ChosenInlineResult {
    result_id: String,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}

#[derive(Deserialize, Debug)]
pub struct CallbackQuery {
    id: String,
    from: User,
    message: Option<Message>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Deserialize, Debug)]
pub struct PreCheckoutQuery {
    id:	String,
    from: User,
    currency: String,
    total_amount: u64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

#[derive(Deserialize, Debug)]
pub struct PollAnswer {
    poll_id: String,
    user: User,
    option_ids: Vec<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ChatMember {
    // TODO
}

#[derive(Deserialize, Debug)]
pub struct ChatInviteLink {
    invite_link: String,
    creator: User,
    creates_join_request: bool,
    is_primary: bool,
    is_revoked: bool,
    name: Option<String>,
    expire_date: Option<u64>,
    member_limit: Option<u64>,
    pending_join_request_count: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ChatMemberUpdated {
    chat: Chat,
    from: User,
    date: u64,
    old_chat_member: ChatMember,
    new_chat_member: ChatMember,
    invite_link: Option<ChatInviteLink>,
}

#[derive(Deserialize, Debug)]
pub struct ChatJoinRequest {
    chat: Chat,
    from: User,
    user_chat_id: i64,
    date: u64,
    bio: Option<String>,
    invite_link: Option<ChatInviteLink>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    pub message: Option<Message>,
    edited_message: Option<Message>,
    channel_post: Option<Message>,
    edited_channel_post: Option<Message>,
    inline_query: Option<InlineQuery>,
    chosen_inline_result: Option<ChosenInlineResult>,
    callback_query: Option<CallbackQuery>,
    shipping_query: Option<ShippingQuery>,
    pre_checkout_query: Option<PreCheckoutQuery>,
    poll: Option<Poll>,
    poll_answer: Option<PollAnswer>,
    my_chat_member: Option<ChatMemberUpdated>,
    chat_member: Option<ChatMemberUpdated>,
    chat_join_request: Option<ChatJoinRequest>,
}