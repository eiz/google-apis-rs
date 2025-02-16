use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View the digital assets you publish on Google Play Movies and TV
    PlaymovyPartnerReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PlaymovyPartnerReadonly => "https://www.googleapis.com/auth/playmovies_partner.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::PlaymovyPartnerReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all PlayMovies related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playmoviespartner1 as playmoviespartner1;
/// use playmoviespartner1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_get("accountId", "availId")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct PlayMovies<> {
    pub client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    pub auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for PlayMovies<> {}

impl<'a, > PlayMovies<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> PlayMovies<> {
        PlayMovies {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/3.0.0".to_string(),
            _base_url: "https://playmoviespartner.googleapis.com/".to_string(),
            _root_url: "https://playmoviespartner.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a> {
        AccountMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/3.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://playmoviespartner.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://playmoviespartner.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// An Order tracks the fulfillment of an Edit when delivered using the
/// legacy, non-component-based delivery.
/// 
/// Each Order is uniquely identified by an `order_id`, which is generated
/// by Google.
/// 
/// Externally, Orders can also be identified by partners using its `custom_id`
/// (when provided).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [orders get accounts](AccountOrderGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// Countries where the Order is available,
    /// using the "ISO 3166-1 alpha-2" format (example: "US").
    pub countries: Option<Vec<String>>,
    /// Detailed status of the order
    #[serde(rename="statusDetail")]
    pub status_detail: Option<String>,
    /// High-level status of the order.
    pub status: Option<String>,
    /// Timestamp of the earliest start date of the Avails
    /// linked to this Order.
    #[serde(rename="earliestAvailStartTime")]
    pub earliest_avail_start_time: Option<String>,
    /// Default Edit name,
    /// usually in the language of the country of origin.
    /// Example: "Googlers, The".
    pub name: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename="studioName")]
    pub studio_name: Option<String>,
    /// Timestamp when the Order was fulfilled.
    #[serde(rename="receivedTime")]
    pub received_time: Option<String>,
    /// Default Season name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - A Brave New World".
    #[serde(rename="seasonName")]
    pub season_name: Option<String>,
    /// ID that can be used to externally identify an Order.
    /// This ID is provided by partners when submitting the Avails.
    /// Example: 'GOOGLER_2006'
    #[serde(rename="customId")]
    pub custom_id: Option<String>,
    /// YouTube Channel Name that should be used to fulfill the Order.
    /// Example: "Google_channel".
    #[serde(rename="channelName")]
    pub channel_name: Option<String>,
    /// Timestamp when the Order was approved.
    #[serde(rename="approvedTime")]
    pub approved_time: Option<String>,
    /// Default Show name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The".
    #[serde(rename="showName")]
    pub show_name: Option<String>,
    /// A simpler representation of the priority.
    #[serde(rename="normalizedPriority")]
    pub normalized_priority: Option<String>,
    /// ID internally generated by Google to uniquely identify an Order.
    /// Example: 'abcde12_x'
    #[serde(rename="orderId")]
    pub order_id: Option<String>,
    /// Type of the Edit linked to the Order.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Field explaining why an Order has been rejected.
    /// Example: "Trailer audio is 2ch mono, please re-deliver in stereo".
    #[serde(rename="rejectionNote")]
    pub rejection_note: Option<String>,
    /// YouTube Channel ID that should be used to fulfill the Order.
    /// Example: "UCRG64darCZhb".
    #[serde(rename="channelId")]
    pub channel_id: Option<String>,
    /// Legacy Order priority, as defined by Google.
    /// Example: 'P0'
    #[serde(rename="legacyPriority")]
    pub legacy_priority: Option<String>,
    /// Name of the post-production house that manages the Edit ordered.
    #[serde(rename="pphName")]
    pub pph_name: Option<String>,
    /// Timestamp when the Order was created.
    #[serde(rename="orderedTime")]
    pub ordered_time: Option<String>,
    /// Order priority, as defined by Google.
    /// The higher the value, the higher the priority.
    /// Example: 90
    pub priority: Option<f64>,
    /// Google-generated ID identifying the video linked to this Order, once
    /// delivered.
    /// Example: 'gtry456_xc'.
    #[serde(rename="videoId")]
    pub video_id: Option<String>,
    /// Default Episode name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - Pilot".
    #[serde(rename="episodeName")]
    pub episode_name: Option<String>,
}

impl client::ResponseResult for Order {}


/// Response to the 'ListStoreInfos' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [store infos list accounts](AccountStoreInfoListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListStoreInfosResponse {
    /// See 'List methods rules' for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// List of StoreInfos that match the request criteria.
    #[serde(rename="storeInfos")]
    pub store_infos: Option<Vec<StoreInfo>>,
}

impl client::ResponseResult for ListStoreInfosResponse {}


/// Response to the 'ListAvails' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [avails list accounts](AccountAvailListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAvailsResponse {
    /// List of Avails that match the request criteria.
    pub avails: Option<Vec<Avail>>,
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListAvailsResponse {}


/// Information about a playable sequence (video) associated with an Edit
/// and available at the Google Play Store.
/// 
/// Internally, each StoreInfo is uniquely identified by a `video_id`
/// and `country`.
/// 
/// Externally, Title-level EIDR or Edit-level EIDR, if provided,
/// can also be used to identify a specific title or edit in a country.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [store infos country get accounts](AccountStoreInfoCountryGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoreInfo {
    /// Timestamp when the Edit went live on the Store.
    #[serde(rename="liveTime")]
    pub live_time: Option<String>,
    /// Google-generated ID identifying the video linked to the Edit.
    /// Example: 'gtry456_xc'
    #[serde(rename="videoId")]
    pub video_id: Option<String>,
    /// Whether the Edit has info cards.
    #[serde(rename="hasInfoCards")]
    pub has_info_cards: Option<bool>,
    /// Whether the Edit has a VOD offer.
    #[serde(rename="hasVodOffer")]
    pub has_vod_offer: Option<bool>,
    /// Name of the post-production houses that manage the Edit.
    #[serde(rename="pphNames")]
    pub pph_names: Option<Vec<String>>,
    /// The number assigned to the episode within a season.
    /// Only available on TV Edits.
    /// Example: "1".
    #[serde(rename="episodeNumber")]
    pub episode_number: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename="studioName")]
    pub studio_name: Option<String>,
    /// Subtitles available for this Edit.
    pub subtitles: Option<Vec<String>>,
    /// Audio tracks available for this Edit.
    #[serde(rename="audioTracks")]
    pub audio_tracks: Option<Vec<String>>,
    /// Default Show name, usually in the language of the country of
    /// origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The".
    #[serde(rename="showName")]
    pub show_name: Option<String>,
    /// Country where Edit is available in ISO 3166-1 alpha-2 country
    /// code.
    /// Example: "US".
    pub country: Option<String>,
    /// Google-generated ID identifying the show linked to the Edit.
    /// Only available for TV Edits.
    /// Example: 'et2hsue_x'
    #[serde(rename="showId")]
    pub show_id: Option<String>,
    /// Edit type, like Movie, Episode or Season.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Google-generated ID identifying the trailer linked to the Edit.
    /// Example: 'bhd_4e_cx'
    #[serde(rename="trailerId")]
    pub trailer_id: Option<String>,
    /// Whether the Edit has a HD offer.
    #[serde(rename="hasHdOffer")]
    pub has_hd_offer: Option<bool>,
    /// Knowledge Graph ID associated to this Edit, if available.
    /// This ID links the Edit to its knowledge entity, externally accessible
    /// at http://freebase.com.
    /// In the absense of Title EIDR or Edit EIDR, this ID helps link together
    /// multiple Edits across countries.
    /// Example: '/m/0ffx29'
    pub mid: Option<String>,
    /// Whether the Edit has a 5.1 channel audio track.
    #[serde(rename="hasAudio51")]
    pub has_audio51: Option<bool>,
    /// Default Edit name, usually in the language of the country of
    /// origin.
    /// Example: "Googlers, The".
    pub name: Option<String>,
    /// Google-generated ID identifying the season linked to the Edit.
    /// Only available for TV Edits.
    /// Example: 'ster23ex'
    #[serde(rename="seasonId")]
    pub season_id: Option<String>,
    /// Title-level EIDR ID.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="titleLevelEidr")]
    pub title_level_eidr: Option<String>,
    /// Default Season name, usually in the language of the country of
    /// origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - A Brave New World".
    #[serde(rename="seasonName")]
    pub season_name: Option<String>,
    /// The number assigned to the season within a show.
    /// Only available on TV Edits.
    /// Example: "1".
    #[serde(rename="seasonNumber")]
    pub season_number: Option<String>,
    /// Whether the Edit has a EST offer.
    #[serde(rename="hasEstOffer")]
    pub has_est_offer: Option<bool>,
    /// Edit-level EIDR ID.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-6".
    #[serde(rename="editLevelEidr")]
    pub edit_level_eidr: Option<String>,
    /// Whether the Edit has a SD offer.
    #[serde(rename="hasSdOffer")]
    pub has_sd_offer: Option<bool>,
}

impl client::ResponseResult for StoreInfo {}


/// An Avail describes the Availability Window of a specific Edit in a given
/// country, which means the period Google is allowed to sell or rent the Edit.
/// 
/// Avails are exposed in EMA format Version 1.6b (available at
/// http://www.movielabs.com/md/avails/)
/// 
/// Studios can see the Avails for the Titles they own.
/// Post-production houses cannot see any Avails.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [avails get accounts](AccountAvailGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Avail {
    /// Title used by involved parties to refer to this series.
    /// Only available on TV Avails.
    /// Example: "Googlers, The".
    #[serde(rename="seriesTitleInternalAlias")]
    pub series_title_internal_alias: Option<String>,
    /// Indicates the format profile covered by the transaction.
    #[serde(rename="formatProfile")]
    pub format_profile: Option<String>,
    /// Title Identifier. This should be the Title Level EIDR.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename="contentId")]
    pub content_id: Option<String>,
    /// Title used by involved parties to refer to this content.
    /// Example: "Googlers, The".
    /// Only available on Movie Avails.
    #[serde(rename="titleInternalAlias")]
    pub title_internal_alias: Option<String>,
    /// Value representing the rating.
    /// Ratings should be formatted as per http://www.movielabs.com/md/ratings/
    /// Example: "PG"
    #[serde(rename="ratingValue")]
    pub rating_value: Option<String>,
    /// Spoken language of the intended audience.
    /// Language shall be encoded in accordance with RFC 5646.
    /// Example: "fr".
    #[serde(rename="storeLanguage")]
    pub store_language: Option<String>,
    /// Communicating an exempt category as defined by FCC regulations.
    /// It is not required for non-US Avails.
    /// Example: "1"
    #[serde(rename="captionExemption")]
    pub caption_exemption: Option<String>,
    /// The name of the studio that owns the Edit referred in the Avail.
    /// This is the equivalent of `studio_name` in other resources, but it follows
    /// the EMA nomenclature.
    /// Example: "Google Films".
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Edit Identifier. This should be the Edit Level EIDR.
    /// Example: "10.2340/1489-49A2-3956-4B2D-FE16-6"
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// Title used by involved parties to refer to this season.
    /// Only available on TV Avails.
    /// Example: "Googlers, The".
    #[serde(rename="seasonTitleInternalAlias")]
    pub season_title_internal_alias: Option<String>,
    /// Other identifier referring to the episode, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers_s1_3".
    #[serde(rename="episodeAltId")]
    pub episode_alt_id: Option<String>,
    /// Value to be applied to the pricing type.
    /// Example: "4" or "2.99"
    #[serde(rename="priceValue")]
    pub price_value: Option<String>,
    /// ISO 3166-1 alpha-2 country code for the country or territory
    /// of this Avail.
    /// For Avails, we use Territory in lieu of Country to comply with
    /// EMA specifications.
    /// But please note that Territory and Country identify the same thing.
    /// Example: "US".
    pub territory: Option<String>,
    /// Work type as enumerated in EMA.
    #[serde(rename="workType")]
    pub work_type: Option<String>,
    /// ID internally generated by Google to uniquely identify an Avail.
    /// Not part of EMA Specs.
    #[serde(rename="availId")]
    pub avail_id: Option<String>,
    /// Value representing the rating reason.
    /// Rating reasons should be formatted as per
    /// [EMA ratings spec](http://www.movielabs.com/md/ratings/)
    /// and comma-separated for inclusion of multiple reasons.
    /// Example: "L, S, V"
    #[serde(rename="ratingReason")]
    pub rating_reason: Option<String>,
    /// OPTIONAL.TV Only. Title used by involved parties to refer to this episode.
    /// Only available on TV Avails.
    /// Example: "Coding at Google".
    #[serde(rename="episodeTitleInternalAlias")]
    pub episode_title_internal_alias: Option<String>,
    /// First date an Edit could be publically announced as becoming
    /// available at a specific future date in territory of Avail.
    /// *Not* the Avail start date or pre-order start date.
    /// Format is YYYY-MM-DD.
    /// Only available for pre-orders.
    /// Example: "2012-12-10"
    #[serde(rename="suppressionLiftDate")]
    pub suppression_lift_date: Option<String>,
    /// Other identifier referring to the season, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers_s1".
    #[serde(rename="seasonAltId")]
    pub season_alt_id: Option<String>,
    /// Manifestation Identifier. This should be the Manifestation
    /// Level EIDR.
    /// Example: "10.2340/1489-49A2-3956-4B2D-FE16-7"
    #[serde(rename="encodeId")]
    pub encode_id: Option<String>,
    /// Type of pricing that should be applied to this Avail
    /// based on how the partner classify them.
    /// Example: "Tier", "WSP", "SRP", or "Category".
    #[serde(rename="priceType")]
    pub price_type: Option<String>,
    /// Communicating if caption file will be delivered.
    #[serde(rename="captionIncluded")]
    pub caption_included: Option<bool>,
    /// Type of transaction.
    #[serde(rename="licenseType")]
    pub license_type: Option<String>,
    /// The number assigned to the season within a series.
    /// Only available on TV Avails.
    /// Example: "1".
    #[serde(rename="seasonNumber")]
    pub season_number: Option<String>,
    /// Release date of the Title in earliest released territory.
    /// Typically it is just the year, but it is free-form as per EMA spec.
    /// Examples: "1979", "Oct 2014"
    #[serde(rename="releaseDate")]
    pub release_date: Option<String>,
    /// End of term in YYYY-MM-DD format in the timezone of the country
    /// of the Avail.
    /// "Open" if no end date is available.
    /// Example: "2019-02-17"
    pub end: Option<String>,
    /// Google-generated ID identifying the video linked to this Avail, once
    /// delivered.
    /// Not part of EMA Specs.
    /// Example: 'gtry456_xc'
    #[serde(rename="videoId")]
    pub video_id: Option<String>,
    /// Start of term in YYYY-MM-DD format in the timezone of the
    /// country of the Avail.
    /// Example: "2013-05-14".
    pub start: Option<String>,
    /// Rating system applied to the version of title within territory
    /// of Avail.
    /// Rating systems should be formatted as per
    /// [EMA ratings spec](http://www.movielabs.com/md/ratings/)
    /// Example: "MPAA"
    #[serde(rename="ratingSystem")]
    pub rating_system: Option<String>,
    /// Name of the post-production houses that manage the Avail.
    /// Not part of EMA Specs.
    #[serde(rename="pphNames")]
    pub pph_names: Option<Vec<String>>,
    /// Other identifier referring to the series, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers".
    #[serde(rename="seriesAltId")]
    pub series_alt_id: Option<String>,
    /// Other identifier referring to the Edit, as defined by partner.
    /// Example: "GOOGLER_2006"
    #[serde(rename="altId")]
    pub alt_id: Option<String>,
    /// The number assigned to the episode within a season.
    /// Only available on TV Avails.
    /// Example: "3".
    #[serde(rename="episodeNumber")]
    pub episode_number: Option<String>,
}

impl client::ResponseResult for Avail {}


/// Response to the 'ListOrders' method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [orders list accounts](AccountOrderListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOrdersResponse {
    /// List of Orders that match the request criteria.
    pub orders: Option<Vec<Order>>,
    /// See _List methods rules_ for info about this field.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
}

impl client::ResponseResult for ListOrdersResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the `PlayMovies` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playmoviespartner1 as playmoviespartner1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `avails_get(...)`, `avails_list(...)`, `orders_get(...)`, `orders_list(...)`, `store_infos_country_get(...)` and `store_infos_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a>
    where  {

    hub: &'a PlayMovies<>,
}

impl<'a> client::MethodsBuilder for AccountMethods<'a> {}

impl<'a> AccountMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Orders owned or managed by the partner.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn orders_list(&self, account_id: &str) -> AccountOrderListCall<'a> {
        AccountOrderListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _studio_names: Default::default(),
            _status: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _custom_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an Order given its id.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _Get methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `orderId` - REQUIRED. Order ID.
    pub fn orders_get(&self, account_id: &str, order_id: &str) -> AccountOrderGetCall<'a> {
        AccountOrderGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Avails owned or managed by the partner.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn avails_list(&self, account_id: &str) -> AccountAvailListCall<'a> {
        AccountAvailListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _title: Default::default(),
            _territories: Default::default(),
            _studio_names: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _alt_ids: Default::default(),
            _alt_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an Avail given its avail group id and avail id.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `availId` - REQUIRED. Avail ID.
    pub fn avails_get(&self, account_id: &str, avail_id: &str) -> AccountAvailGetCall<'a> {
        AccountAvailGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _avail_id: avail_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a StoreInfo given its video id and country.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _Get methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `videoId` - REQUIRED. Video ID.
    /// * `country` - REQUIRED. Edit country.
    pub fn store_infos_country_get(&self, account_id: &str, video_id: &str, country: &str) -> AccountStoreInfoCountryGetCall<'a> {
        AccountStoreInfoCountryGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_id: video_id.to_string(),
            _country: country.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List StoreInfos owned or managed by the partner.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn store_infos_list(&self, account_id: &str) -> AccountStoreInfoListCall<'a> {
        AccountStoreInfoListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _video_id: Default::default(),
            _studio_names: Default::default(),
            _season_ids: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _mids: Default::default(),
            _countries: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// List Orders owned or managed by the partner.
/// 
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *orders.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_list("accountId")
///              .add_video_ids("sed")
///              .add_studio_names("amet.")
///              .add_status("takimata")
///              .add_pph_names("amet.")
///              .page_token("duo")
///              .page_size(-55)
///              .name("gubergren")
///              .custom_id("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct AccountOrderListCall<'a>
    where  {

    hub: &'a PlayMovies<>,
    _account_id: String,
    _video_ids: Vec<String>,
    _studio_names: Vec<String>,
    _status: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _name: Option<String>,
    _custom_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for AccountOrderListCall<'a> {}

impl<'a> AccountOrderListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListOrdersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.orders.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("accountId", self._account_id.to_string()));
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push(("videoIds", f.to_string()));
            }
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._status.len() > 0 {
            for f in self._status.iter() {
                params.push(("status", f.to_string()));
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._custom_id {
            params.push(("customId", value.to_string()));
        }
        for &field in ["alt", "accountId", "videoIds", "studioNames", "status", "pphNames", "pageToken", "pageSize", "name", "customId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/orders";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Orders that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter Orders that match one of the given status.
    ///
    /// Append the given value to the *status* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_status(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._status.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountOrderListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches Orders with a `name`, `show`, `season` or `episode`
    /// that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter Orders that match a case-insensitive, partner-specific custom id.
    ///
    /// Sets the *custom id* query property to the given value.
    pub fn custom_id(mut self, new_value: &str) -> AccountOrderListCall<'a> {
        self._custom_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountOrderListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountOrderListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get an Order given its id.
/// 
/// See _Authentication and Authorization rules_ and
/// _Get methods rules_ for more information about this method.
///
/// A builder for the *orders.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_get("accountId", "orderId")
///              .doit().await;
/// # }
/// ```
pub struct AccountOrderGetCall<'a>
    where  {

    hub: &'a PlayMovies<>,
    _account_id: String,
    _order_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for AccountOrderGetCall<'a> {}

impl<'a> AccountOrderGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Order)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.orders.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("accountId", self._account_id.to_string()));
        params.push(("orderId", self._order_id.to_string()));
        for &field in ["alt", "accountId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/orders/{orderId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{orderId}", "orderId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["orderId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderGetCall<'a> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Order ID.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> AccountOrderGetCall<'a> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountOrderGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountOrderGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List Avails owned or managed by the partner.
/// 
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *avails.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_list("accountId")
///              .add_video_ids("ea")
///              .title("ipsum")
///              .add_territories("invidunt")
///              .add_studio_names("amet")
///              .add_pph_names("duo")
///              .page_token("ipsum")
///              .page_size(-93)
///              .add_alt_ids("ut")
///              .alt_id("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct AccountAvailListCall<'a>
    where  {

    hub: &'a PlayMovies<>,
    _account_id: String,
    _video_ids: Vec<String>,
    _title: Option<String>,
    _territories: Vec<String>,
    _studio_names: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _alt_ids: Vec<String>,
    _alt_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for AccountAvailListCall<'a> {}

impl<'a> AccountAvailListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListAvailsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.avails.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(12 + self._additional_params.len());
        params.push(("accountId", self._account_id.to_string()));
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push(("videoIds", f.to_string()));
            }
        }
        if let Some(value) = self._title {
            params.push(("title", value.to_string()));
        }
        if self._territories.len() > 0 {
            for f in self._territories.iter() {
                params.push(("territories", f.to_string()));
            }
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if self._alt_ids.len() > 0 {
            for f in self._alt_ids.iter() {
                params.push(("altIds", f.to_string()));
            }
        }
        if let Some(value) = self._alt_id {
            params.push(("altId", value.to_string()));
        }
        for &field in ["alt", "accountId", "videoIds", "title", "territories", "studioNames", "pphNames", "pageToken", "pageSize", "altIds", "altId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/avails";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Avails that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter that matches Avails with a `title_internal_alias`,
    /// `series_title_internal_alias`, `season_title_internal_alias`,
    /// or `episode_title_internal_alias` that contains the given
    /// case-insensitive title.
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given country codes,
    /// using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *territories* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_territories(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._territories.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountAvailListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given partner-specific custom ids.
    ///
    /// Append the given value to the *alt ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_alt_ids(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._alt_ids.push(new_value.to_string());
        self
    }
    /// Filter Avails that match a case-insensitive, partner-specific custom id.
    /// NOTE: this field is deprecated and will be removed on V2; `alt_ids`
    /// should be used instead.
    ///
    /// Sets the *alt id* query property to the given value.
    pub fn alt_id(mut self, new_value: &str) -> AccountAvailListCall<'a> {
        self._alt_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountAvailListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountAvailListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get an Avail given its avail group id and avail id.
///
/// A builder for the *avails.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_get("accountId", "availId")
///              .doit().await;
/// # }
/// ```
pub struct AccountAvailGetCall<'a>
    where  {

    hub: &'a PlayMovies<>,
    _account_id: String,
    _avail_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for AccountAvailGetCall<'a> {}

impl<'a> AccountAvailGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Avail)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.avails.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("accountId", self._account_id.to_string()));
        params.push(("availId", self._avail_id.to_string()));
        for &field in ["alt", "accountId", "availId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/avails/{availId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{availId}", "availId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["availId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailGetCall<'a> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Avail ID.
    ///
    /// Sets the *avail id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn avail_id(mut self, new_value: &str) -> AccountAvailGetCall<'a> {
        self._avail_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountAvailGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountAvailGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get a StoreInfo given its video id and country.
/// 
/// See _Authentication and Authorization rules_ and
/// _Get methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.country.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_country_get("accountId", "videoId", "country")
///              .doit().await;
/// # }
/// ```
pub struct AccountStoreInfoCountryGetCall<'a>
    where  {

    hub: &'a PlayMovies<>,
    _account_id: String,
    _video_id: String,
    _country: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for AccountStoreInfoCountryGetCall<'a> {}

impl<'a> AccountStoreInfoCountryGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, StoreInfo)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.storeInfos.country.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("accountId", self._account_id.to_string()));
        params.push(("videoId", self._video_id.to_string()));
        params.push(("country", self._country.to_string()));
        for &field in ["alt", "accountId", "videoId", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/storeInfos/{videoId}/country/{country}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{videoId}", "videoId"), ("{country}", "country")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["country", "videoId", "accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Video ID.
    ///
    /// Sets the *video id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a> {
        self._video_id = new_value.to_string();
        self
    }
    /// REQUIRED. Edit country.
    ///
    /// Sets the *country* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn country(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a> {
        self._country = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountStoreInfoCountryGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoCountryGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountStoreInfoCountryGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List StoreInfos owned or managed by the partner.
/// 
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_list("accountId")
///              .add_video_ids("ea")
///              .video_id("dolor")
///              .add_studio_names("Lorem")
///              .add_season_ids("eos")
///              .add_pph_names("labore")
///              .page_token("sed")
///              .page_size(-70)
///              .name("sed")
///              .add_mids("no")
///              .add_countries("Stet")
///              .doit().await;
/// # }
/// ```
pub struct AccountStoreInfoListCall<'a>
    where  {

    hub: &'a PlayMovies<>,
    _account_id: String,
    _video_ids: Vec<String>,
    _video_id: Option<String>,
    _studio_names: Vec<String>,
    _season_ids: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _name: Option<String>,
    _mids: Vec<String>,
    _countries: Vec<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for AccountStoreInfoListCall<'a> {}

impl<'a> AccountStoreInfoListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListStoreInfosResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.storeInfos.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("accountId", self._account_id.to_string()));
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push(("videoIds", f.to_string()));
            }
        }
        if let Some(value) = self._video_id {
            params.push(("videoId", value.to_string()));
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push(("studioNames", f.to_string()));
            }
        }
        if self._season_ids.len() > 0 {
            for f in self._season_ids.iter() {
                params.push(("seasonIds", f.to_string()));
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push(("pphNames", f.to_string()));
            }
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if self._mids.len() > 0 {
            for f in self._mids.iter() {
                params.push(("mids", f.to_string()));
            }
        }
        if self._countries.len() > 0 {
            for f in self._countries.iter() {
                params.push(("countries", f.to_string()));
            }
        }
        for &field in ["alt", "accountId", "videoIds", "videoId", "studioNames", "seasonIds", "pphNames", "pageToken", "pageSize", "name", "mids", "countries"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/storeInfos";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["accountId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter StoreInfos that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match a given `video_id`.
    /// NOTE: this field is deprecated and will be removed on V2; `video_ids`
    /// should be used instead.
    ///
    /// Sets the *video id* query property to the given value.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._video_id = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `season_id`s.
    ///
    /// Append the given value to the *season ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_season_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._season_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountStoreInfoListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches StoreInfos with a `name` or `show_name`
    /// that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `mid`s.
    ///
    /// Append the given value to the *mids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_mids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._mids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match (case-insensitive) any of the given country
    /// codes, using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *countries* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_countries(mut self, new_value: &str) -> AccountStoreInfoListCall<'a> {
        self._countries.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountStoreInfoListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::PlaymovyPartnerReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountStoreInfoListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


