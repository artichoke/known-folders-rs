// src/win.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::c_void;
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::slice;

use windows_sys::core::{GUID, PWSTR};
use windows_sys::Win32::{
    Foundation::{E_FAIL, E_INVALIDARG, HANDLE, S_OK},
    Globalization::lstrlenW,
    System::Com::CoTaskMemFree,
    UI::Shell::{SHGetKnownFolderPath, KF_FLAG_DEFAULT},
};

use windows_sys::Win32::UI::Shell::{
    FOLDERID_AccountPictures, FOLDERID_AddNewPrograms, FOLDERID_AdminTools, FOLDERID_AllAppMods,
    FOLDERID_AppCaptures, FOLDERID_AppDataDesktop, FOLDERID_AppDataDocuments,
    FOLDERID_AppDataFavorites, FOLDERID_AppDataProgramData, FOLDERID_AppUpdates,
    FOLDERID_ApplicationShortcuts, FOLDERID_AppsFolder, FOLDERID_CDBurning, FOLDERID_CameraRoll,
    FOLDERID_CameraRollLibrary, FOLDERID_ChangeRemovePrograms, FOLDERID_CommonAdminTools,
    FOLDERID_CommonOEMLinks, FOLDERID_CommonPrograms, FOLDERID_CommonStartMenu,
    FOLDERID_CommonStartMenuPlaces, FOLDERID_CommonStartup, FOLDERID_CommonTemplates,
    FOLDERID_ComputerFolder, FOLDERID_ConflictFolder, FOLDERID_ConnectionsFolder,
    FOLDERID_Contacts, FOLDERID_ControlPanelFolder, FOLDERID_Cookies, FOLDERID_CurrentAppMods,
    FOLDERID_Desktop, FOLDERID_DevelopmentFiles, FOLDERID_Device, FOLDERID_DeviceMetadataStore,
    FOLDERID_Documents, FOLDERID_DocumentsLibrary, FOLDERID_Downloads, FOLDERID_Favorites,
    FOLDERID_Fonts, FOLDERID_GameTasks, FOLDERID_Games, FOLDERID_History, FOLDERID_HomeGroup,
    FOLDERID_HomeGroupCurrentUser, FOLDERID_ImplicitAppShortcuts, FOLDERID_InternetCache,
    FOLDERID_InternetFolder, FOLDERID_Libraries, FOLDERID_Links, FOLDERID_LocalAppData,
    FOLDERID_LocalAppDataLow, FOLDERID_LocalDocuments, FOLDERID_LocalDownloads,
    FOLDERID_LocalMusic, FOLDERID_LocalPictures, FOLDERID_LocalStorage, FOLDERID_LocalVideos,
    FOLDERID_LocalizedResourcesDir, FOLDERID_Music, FOLDERID_MusicLibrary, FOLDERID_NetHood,
    FOLDERID_NetworkFolder, FOLDERID_Objects3D, FOLDERID_OneDrive, FOLDERID_OriginalImages,
    FOLDERID_PhotoAlbums, FOLDERID_Pictures, FOLDERID_PicturesLibrary, FOLDERID_Playlists,
    FOLDERID_PrintHood, FOLDERID_PrintersFolder, FOLDERID_Profile, FOLDERID_ProgramData,
    FOLDERID_ProgramFiles, FOLDERID_ProgramFilesCommon, FOLDERID_ProgramFilesCommonX64,
    FOLDERID_ProgramFilesCommonX86, FOLDERID_ProgramFilesX64, FOLDERID_ProgramFilesX86,
    FOLDERID_Programs, FOLDERID_Public, FOLDERID_PublicDesktop, FOLDERID_PublicDocuments,
    FOLDERID_PublicDownloads, FOLDERID_PublicGameTasks, FOLDERID_PublicLibraries,
    FOLDERID_PublicMusic, FOLDERID_PublicPictures, FOLDERID_PublicRingtones,
    FOLDERID_PublicUserTiles, FOLDERID_PublicVideos, FOLDERID_QuickLaunch, FOLDERID_Recent,
    FOLDERID_RecordedCalls, FOLDERID_RecordedTVLibrary, FOLDERID_RecycleBinFolder,
    FOLDERID_ResourceDir, FOLDERID_RetailDemo, FOLDERID_Ringtones, FOLDERID_RoamedTileImages,
    FOLDERID_RoamingAppData, FOLDERID_RoamingTiles, FOLDERID_SampleMusic, FOLDERID_SamplePictures,
    FOLDERID_SamplePlaylists, FOLDERID_SampleVideos, FOLDERID_SavedGames, FOLDERID_SavedPictures,
    FOLDERID_SavedPicturesLibrary, FOLDERID_SavedSearches, FOLDERID_Screenshots,
    FOLDERID_SearchHistory, FOLDERID_SearchHome, FOLDERID_SearchTemplates, FOLDERID_SendTo,
    FOLDERID_SidebarDefaultParts, FOLDERID_SidebarParts, FOLDERID_SkyDrive,
    FOLDERID_SkyDriveCameraRoll, FOLDERID_SkyDriveDocuments, FOLDERID_SkyDriveMusic,
    FOLDERID_SkyDrivePictures, FOLDERID_StartMenu, FOLDERID_StartMenuAllPrograms, FOLDERID_Startup,
    FOLDERID_SyncManagerFolder, FOLDERID_SyncResultsFolder, FOLDERID_SyncSetupFolder,
    FOLDERID_System, FOLDERID_SystemX86, FOLDERID_Templates, FOLDERID_UserPinned,
    FOLDERID_UserProfiles, FOLDERID_UserProgramFiles, FOLDERID_UserProgramFilesCommon,
    FOLDERID_UsersFiles, FOLDERID_UsersLibraries, FOLDERID_Videos, FOLDERID_VideosLibrary,
    FOLDERID_Windows, FOLDERID_SEARCH_CSC, FOLDERID_SEARCH_MAPI,
};

mod guard {
    pub struct FreeGuard(PWSTR);

    impl FreeGuard {
        /// Per upstream documentation, the last parameter to
        /// `SHGetKnownFolderPath` is a pointer to a pointer.
        ///
        /// # Parameter
        ///
        /// > `[out] ppszPath`
        /// >
        /// > Type: `PWSTR*`
        ///
        /// https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters
        ///
        /// `PWSTR` itself is a `*mut u16`:
        ///
        /// https://docs.rs/windows-sys/0.48.0/windows_sys/core/type.PWSTR.html
        pub fn as_out_ppszPath(&mut self) -> &mut PWSTR {
            &mut self.0
        }

        /// Access the inner wide string.
        pub fn as_pwstr(&self) -> PWSTR {
            self.0
        }
    }

    impl Default for FreeGuard {
        fn default() -> Self {
            let ptr = ptr::null_mut::<PWSTR>();
            Self(ptr)
        }
    }

    impl Drop for FreeGuard {
        fn drop(&mut self) {
            let ptr = self.0.cast::<c_void>();
            // SAFETY: `ptr` must always be freed per the API documentation:
            //
            // > The calling process is responsible for freeing this resource
            // > once it is no longer needed by calling `CoTaskMemFree`, whether
            // > `SHGetKnownFolderPath` succeeds or not.
            unsafe {
                CoTaskMemFree(ptr);
            }
        }
    }
}

/// GUIDs that identify standard folders registered with the system as
/// [Known Folders].
///
/// These folders are installed with Windows Vista and later operating systems,
/// and a computer will have only folders appropriate to it installed.
///
/// For details on the **KNOWNFOLDERID** constants this enum represents, please
/// refer to the [upstream documentaion].
///
/// # Compatibility Notes
///
/// The Known Folders API allows for ISVs to extend the set of Known Folder IDs,
/// but this enum only has support for first-party Known Folder IDs included in
/// [`windows_sys`].
///
/// # Examples
///
/// ```
/// use known_folders::{get_known_folder_path, KnownFolder};
///
/// let profile_dir = get_known_folder_path(KnownFolder::Profile);
/// ```
///
/// [Known Folders]: https://learn.microsoft.com/en-us/windows/win32/shell/known-folders
/// [upstream documentation]: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#constants
#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KnownFolder {
    /// `FOLDERID_AccountPictures`
    AccountPictures,
    /// `FOLDERID_AddNewPrograms`
    AddNewPrograms,
    /// `FOLDERID_AdminTools`
    AdminTools,
    /// `FOLDERID_AllAppMods`
    AllAppMods,
    /// `FOLDERID_AppCaptures`
    AppCaptures,
    /// `FOLDERID_AppDataDesktop`
    AppDataDesktop,
    /// `FOLDERID_AppDataDocuments`
    AppDataDocuments,
    /// `FOLDERID_AppDataFavorites`
    AppDataFavorites,
    /// `FOLDERID_AppDataProgramData`
    AppDataProgramData,
    /// `FOLDERID_AppUpdates`
    AppUpdates,
    /// `FOLDERID_ApplicationShortcuts`
    ApplicationShortcuts,
    /// `FOLDERID_AppsFolder`
    AppsFolder,
    /// `FOLDERID_CDBurning`
    CDBurning,
    /// `FOLDERID_CameraRoll`
    CameraRoll,
    /// `FOLDERID_CameraRollLibrary`
    CameraRollLibrary,
    /// `FOLDERID_ChangeRemovePrograms`
    ChangeRemovePrograms,
    /// `FOLDERID_CommonAdminTools`
    CommonAdminTools,
    /// `FOLDERID_CommonOEMLinks`
    CommonOEMLinks,
    /// `FOLDERID_CommonPrograms`
    CommonPrograms,
    /// `FOLDERID_CommonStartMenu`
    CommonStartMenu,
    /// `FOLDERID_CommonStartMenuPlaces`
    CommonStartMenuPlaces,
    /// `FOLDERID_CommonStartup`
    CommonStartup,
    /// `FOLDERID_CommonTemplates`
    CommonTemplates,
    /// `FOLDERID_ComputerFolder`
    ComputerFolder,
    /// `FOLDERID_ConflictFolder`
    ConflictFolder,
    /// `FOLDERID_ConnectionsFolder`
    ConnectionsFolder,
    /// `FOLDERID_Contacts`
    Contacts,
    /// `FOLDERID_ControlPanelFolder`
    ControlPanelFolder,
    /// `FOLDERID_Cookies`
    Cookies,
    /// `FOLDERID_CurrentAppMods`
    CurrentAppMods,
    /// `FOLDERID_Desktop`
    Desktop,
    /// `FOLDERID_DevelopmentFiles`
    DevelopmentFiles,
    /// `FOLDERID_Device`
    Device,
    /// `FOLDERID_DeviceMetadataStore`
    DeviceMetadataStore,
    /// `FOLDERID_Documents`
    Documents,
    /// `FOLDERID_DocumentsLibrary`
    DocumentsLibrary,
    /// `FOLDERID_Downloads`
    Downloads,
    /// `FOLDERID_Favorites`
    Favorites,
    /// `FOLDERID_Fonts`
    Fonts,
    /// `FOLDERID_GameTasks`
    GameTasks,
    /// `FOLDERID_Games`
    Games,
    /// `FOLDERID_History`
    History,
    /// `FOLDERID_HomeGroup`
    HomeGroup,
    /// `FOLDERID_HomeGroupCurrentUser`
    HomeGroupCurrentUser,
    /// `FOLDERID_ImplicitAppShortcuts`
    ImplicitAppShortcuts,
    /// `FOLDERID_InternetCache`
    InternetCache,
    /// `FOLDERID_InternetFolder`
    InternetFolder,
    /// `FOLDERID_Libraries`
    Libraries,
    /// `FOLDERID_Links`
    Links,
    /// `FOLDERID_LocalAppData`
    LocalAppData,
    /// `FOLDERID_LocalAppDataLow`
    LocalAppDataLow,
    /// `FOLDERID_LocalDocuments`
    LocalDocuments,
    /// `FOLDERID_LocalDownloads`
    LocalDownloads,
    /// `FOLDERID_LocalMusic`
    LocalMusic,
    /// `FOLDERID_LocalPictures`
    LocalPictures,
    /// `FOLDERID_LocalStorage`
    LocalStorage,
    /// `FOLDERID_LocalVideos`
    LocalVideos,
    /// `FOLDERID_LocalizedResourcesDir`
    LocalizedResourcesDir,
    /// `FOLDERID_Music`
    Music,
    /// `FOLDERID_MusicLibrary`
    MusicLibrary,
    /// `FOLDERID_NetHood`
    NetHood,
    /// `FOLDERID_NetworkFolder`
    NetworkFolder,
    /// `FOLDERID_Objects3D`
    Objects3D,
    /// `FOLDERID_OneDrive`
    OneDrive,
    /// `FOLDERID_OriginalImages`
    OriginalImages,
    /// `FOLDERID_PhotoAlbums`
    PhotoAlbums,
    /// `FOLDERID_Pictures`
    Pictures,
    /// `FOLDERID_PicturesLibrary`
    PicturesLibrary,
    /// `FOLDERID_Playlists`
    Playlists,
    /// `FOLDERID_PrintHood`
    PrintHood,
    /// `FOLDERID_PrintersFolder`
    PrintersFolder,
    /// `FOLDERID_Profile`
    Profile,
    /// `FOLDERID_ProgramData`
    ProgramData,
    /// `FOLDERID_ProgramFiles`
    ProgramFiles,
    /// `FOLDERID_ProgramFilesCommon`
    ProgramFilesCommon,
    /// `FOLDERID_ProgramFilesCommonX64`
    ProgramFilesCommonX64,
    /// `FOLDERID_ProgramFilesCommonX86`
    ProgramFilesCommonX86,
    /// `FOLDERID_ProgramFilesX64`
    ProgramFilesX64,
    /// `FOLDERID_ProgramFilesX86`
    ProgramFilesX86,
    /// `FOLDERID_Programs`
    Programs,
    /// `FOLDERID_Public`
    Public,
    /// `FOLDERID_PublicDesktop`
    PublicDesktop,
    /// `FOLDERID_PublicDocuments`
    PublicDocuments,
    /// `FOLDERID_PublicDownloads`
    PublicDownloads,
    /// `FOLDERID_PublicGameTasks`
    PublicGameTasks,
    /// `FOLDERID_PublicLibraries`
    PublicLibraries,
    /// `FOLDERID_PublicMusic`
    PublicMusic,
    /// `FOLDERID_PublicPictures`
    PublicPictures,
    /// `FOLDERID_PublicRingtones`
    PublicRingtones,
    /// `FOLDERID_PublicUserTiles`
    PublicUserTiles,
    /// `FOLDERID_PublicVideos`
    PublicVideos,
    /// `FOLDERID_QuickLaunch`
    QuickLaunch,
    /// `FOLDERID_Recent`
    Recent,
    /// `FOLDERID_RecordedCalls`
    RecordedCalls,
    /// `FOLDERID_RecordedTVLibrary`
    RecordedTVLibrary,
    /// `FOLDERID_RecycleBinFolder`
    RecycleBinFolder,
    /// `FOLDERID_ResourceDir`
    ResourceDir,
    /// `FOLDERID_RetailDemo`
    RetailDemo,
    /// `FOLDERID_Ringtones`
    Ringtones,
    /// `FOLDERID_RoamedTileImages`
    RoamedTileImages,
    /// `FOLDERID_RoamingAppData`
    RoamingAppData,
    /// `FOLDERID_RoamingTiles`
    RoamingTiles,
    /// `FOLDERID_SEARCH_CSC`
    SEARCH_CSC,
    /// `FOLDERID_SEARCH_MAPI`
    SEARCH_MAPI,
    /// `FOLDERID_SampleMusic`
    SampleMusic,
    /// `FOLDERID_SamplePictures`
    SamplePictures,
    /// `FOLDERID_SamplePlaylists`
    SamplePlaylists,
    /// `FOLDERID_SampleVideos`
    SampleVideos,
    /// `FOLDERID_SavedGames`
    SavedGames,
    /// `FOLDERID_SavedPictures`
    SavedPictures,
    /// `FOLDERID_SavedPicturesLibrary`
    SavedPicturesLibrary,
    /// `FOLDERID_SavedSearches`
    SavedSearches,
    /// `FOLDERID_Screenshots`
    Screenshots,
    /// `FOLDERID_SearchHistory`
    SearchHistory,
    /// `FOLDERID_SearchHome`
    SearchHome,
    /// `FOLDERID_SearchTemplates`
    SearchTemplates,
    /// `FOLDERID_SendTo`
    SendTo,
    /// `FOLDERID_SidebarDefaultParts`
    SidebarDefaultParts,
    /// `FOLDERID_SidebarParts`
    SidebarParts,
    /// `FOLDERID_SkyDrive`
    SkyDrive,
    /// `FOLDERID_SkyDriveCameraRoll`
    SkyDriveCameraRoll,
    /// `FOLDERID_SkyDriveDocuments`
    SkyDriveDocuments,
    /// `FOLDERID_SkyDriveMusic`
    SkyDriveMusic,
    /// `FOLDERID_SkyDrivePictures`
    SkyDrivePictures,
    /// `FOLDERID_StartMenu`
    StartMenu,
    /// `FOLDERID_StartMenuAllPrograms`
    StartMenuAllPrograms,
    /// `FOLDERID_Startup`
    Startup,
    /// `FOLDERID_SyncManagerFolder`
    SyncManagerFolder,
    /// `FOLDERID_SyncResultsFolder`
    SyncResultsFolder,
    /// `FOLDERID_SyncSetupFolder`
    SyncSetupFolder,
    /// `FOLDERID_System`
    System,
    /// `FOLDERID_SystemX86`
    SystemX86,
    /// `FOLDERID_Templates`
    Templates,
    /// `FOLDERID_UserPinned`
    UserPinned,
    /// `FOLDERID_UserProfiles`
    UserProfiles,
    /// `FOLDERID_UserProgramFiles`
    UserProgramFiles,
    /// `FOLDERID_UserProgramFilesCommon`
    UserProgramFilesCommon,
    /// `FOLDERID_UsersFiles`
    UsersFiles,
    /// `FOLDERID_UsersLibraries`
    UsersLibraries,
    /// `FOLDERID_Videos`
    Videos,
    /// `FOLDERID_VideosLibrary`
    VideosLibrary,
    /// `FOLDERID_Windows`
    Windows,
}

impl KnownFolder {
    const fn to_guid(self) -> &'static GUID {
        match self {
            Self::AccountPictures => &FOLDERID_AccountPictures,
            Self::AddNewPrograms => &FOLDERID_AddNewPrograms,
            Self::AdminTools => &FOLDERID_AdminTools,
            Self::AllAppMods => &FOLDERID_AllAppMods,
            Self::AppCaptures => &FOLDERID_AppCaptures,
            Self::AppDataDesktop => &FOLDERID_AppDataDesktop,
            Self::AppDataDocuments => &FOLDERID_AppDataDocuments,
            Self::AppDataFavorites => &FOLDERID_AppDataFavorites,
            Self::AppDataProgramData => &FOLDERID_AppDataProgramData,
            Self::AppUpdates => &FOLDERID_AppUpdates,
            Self::ApplicationShortcuts => &FOLDERID_ApplicationShortcuts,
            Self::AppsFolder => &FOLDERID_AppsFolder,
            Self::CDBurning => &FOLDERID_CDBurning,
            Self::CameraRoll => &FOLDERID_CameraRoll,
            Self::CameraRollLibrary => &FOLDERID_CameraRollLibrary,
            Self::ChangeRemovePrograms => &FOLDERID_ChangeRemovePrograms,
            Self::CommonAdminTools => &FOLDERID_CommonAdminTools,
            Self::CommonOEMLinks => &FOLDERID_CommonOEMLinks,
            Self::CommonPrograms => &FOLDERID_CommonPrograms,
            Self::CommonStartMenu => &FOLDERID_CommonStartMenu,
            Self::CommonStartMenuPlaces => &FOLDERID_CommonStartMenuPlaces,
            Self::CommonStartup => &FOLDERID_CommonStartup,
            Self::CommonTemplates => &FOLDERID_CommonTemplates,
            Self::ComputerFolder => &FOLDERID_ComputerFolder,
            Self::ConflictFolder => &FOLDERID_ConflictFolder,
            Self::ConnectionsFolder => &FOLDERID_ConnectionsFolder,
            Self::Contacts => &FOLDERID_Contacts,
            Self::ControlPanelFolder => &FOLDERID_ControlPanelFolder,
            Self::Cookies => &FOLDERID_Cookies,
            Self::CurrentAppMods => &FOLDERID_CurrentAppMods,
            Self::Desktop => &FOLDERID_Desktop,
            Self::DevelopmentFiles => &FOLDERID_DevelopmentFiles,
            Self::Device => &FOLDERID_Device,
            Self::DeviceMetadataStore => &FOLDERID_DeviceMetadataStore,
            Self::Documents => &FOLDERID_Documents,
            Self::DocumentsLibrary => &FOLDERID_DocumentsLibrary,
            Self::Downloads => &FOLDERID_Downloads,
            Self::Favorites => &FOLDERID_Favorites,
            Self::Fonts => &FOLDERID_Fonts,
            Self::GameTasks => &FOLDERID_GameTasks,
            Self::Games => &FOLDERID_Games,
            Self::History => &FOLDERID_History,
            Self::HomeGroup => &FOLDERID_HomeGroup,
            Self::HomeGroupCurrentUser => &FOLDERID_HomeGroupCurrentUser,
            Self::ImplicitAppShortcuts => &FOLDERID_ImplicitAppShortcuts,
            Self::InternetCache => &FOLDERID_InternetCache,
            Self::InternetFolder => &FOLDERID_InternetFolder,
            Self::Libraries => &FOLDERID_Libraries,
            Self::Links => &FOLDERID_Links,
            Self::LocalAppData => &FOLDERID_LocalAppData,
            Self::LocalAppDataLow => &FOLDERID_LocalAppDataLow,
            Self::LocalDocuments => &FOLDERID_LocalDocuments,
            Self::LocalDownloads => &FOLDERID_LocalDownloads,
            Self::LocalMusic => &FOLDERID_LocalMusic,
            Self::LocalPictures => &FOLDERID_LocalPictures,
            Self::LocalStorage => &FOLDERID_LocalStorage,
            Self::LocalVideos => &FOLDERID_LocalVideos,
            Self::LocalizedResourcesDir => &FOLDERID_LocalizedResourcesDir,
            Self::Music => &FOLDERID_Music,
            Self::MusicLibrary => &FOLDERID_MusicLibrary,
            Self::NetHood => &FOLDERID_NetHood,
            Self::NetworkFolder => &FOLDERID_NetworkFolder,
            Self::Objects3D => &FOLDERID_Objects3D,
            Self::OneDrive => &FOLDERID_OneDrive,
            Self::OriginalImages => &FOLDERID_OriginalImages,
            Self::PhotoAlbums => &FOLDERID_PhotoAlbums,
            Self::Pictures => &FOLDERID_Pictures,
            Self::PicturesLibrary => &FOLDERID_PicturesLibrary,
            Self::Playlists => &FOLDERID_Playlists,
            Self::PrintHood => &FOLDERID_PrintHood,
            Self::PrintersFolder => &FOLDERID_PrintersFolder,
            Self::Profile => &FOLDERID_Profile,
            Self::ProgramData => &FOLDERID_ProgramData,
            Self::ProgramFiles => &FOLDERID_ProgramFiles,
            Self::ProgramFilesCommon => &FOLDERID_ProgramFilesCommon,
            Self::ProgramFilesCommonX64 => &FOLDERID_ProgramFilesCommonX64,
            Self::ProgramFilesCommonX86 => &FOLDERID_ProgramFilesCommonX86,
            Self::ProgramFilesX64 => &FOLDERID_ProgramFilesX64,
            Self::ProgramFilesX86 => &FOLDERID_ProgramFilesX86,
            Self::Programs => &FOLDERID_Programs,
            Self::Public => &FOLDERID_Public,
            Self::PublicDesktop => &FOLDERID_PublicDesktop,
            Self::PublicDocuments => &FOLDERID_PublicDocuments,
            Self::PublicDownloads => &FOLDERID_PublicDownloads,
            Self::PublicGameTasks => &FOLDERID_PublicGameTasks,
            Self::PublicLibraries => &FOLDERID_PublicLibraries,
            Self::PublicMusic => &FOLDERID_PublicMusic,
            Self::PublicPictures => &FOLDERID_PublicPictures,
            Self::PublicRingtones => &FOLDERID_PublicRingtones,
            Self::PublicUserTiles => &FOLDERID_PublicUserTiles,
            Self::PublicVideos => &FOLDERID_PublicVideos,
            Self::QuickLaunch => &FOLDERID_QuickLaunch,
            Self::Recent => &FOLDERID_Recent,
            Self::RecordedCalls => &FOLDERID_RecordedCalls,
            Self::RecordedTVLibrary => &FOLDERID_RecordedTVLibrary,
            Self::RecycleBinFolder => &FOLDERID_RecycleBinFolder,
            Self::ResourceDir => &FOLDERID_ResourceDir,
            Self::RetailDemo => &FOLDERID_RetailDemo,
            Self::Ringtones => &FOLDERID_Ringtones,
            Self::RoamedTileImages => &FOLDERID_RoamedTileImages,
            Self::RoamingAppData => &FOLDERID_RoamingAppData,
            Self::RoamingTiles => &FOLDERID_RoamingTiles,
            Self::SEARCH_CSC => &FOLDERID_SEARCH_CSC,
            Self::SEARCH_MAPI => &FOLDERID_SEARCH_MAPI,
            Self::SampleMusic => &FOLDERID_SampleMusic,
            Self::SamplePictures => &FOLDERID_SamplePictures,
            Self::SamplePlaylists => &FOLDERID_SamplePlaylists,
            Self::SampleVideos => &FOLDERID_SampleVideos,
            Self::SavedGames => &FOLDERID_SavedGames,
            Self::SavedPictures => &FOLDERID_SavedPictures,
            Self::SavedPicturesLibrary => &FOLDERID_SavedPicturesLibrary,
            Self::SavedSearches => &FOLDERID_SavedSearches,
            Self::Screenshots => &FOLDERID_Screenshots,
            Self::SearchHistory => &FOLDERID_SearchHistory,
            Self::SearchHome => &FOLDERID_SearchHome,
            Self::SearchTemplates => &FOLDERID_SearchTemplates,
            Self::SendTo => &FOLDERID_SendTo,
            Self::SidebarDefaultParts => &FOLDERID_SidebarDefaultParts,
            Self::SidebarParts => &FOLDERID_SidebarParts,
            Self::SkyDrive => &FOLDERID_SkyDrive,
            Self::SkyDriveCameraRoll => &FOLDERID_SkyDriveCameraRoll,
            Self::SkyDriveDocuments => &FOLDERID_SkyDriveDocuments,
            Self::SkyDriveMusic => &FOLDERID_SkyDriveMusic,
            Self::SkyDrivePictures => &FOLDERID_SkyDrivePictures,
            Self::StartMenu => &FOLDERID_StartMenu,
            Self::StartMenuAllPrograms => &FOLDERID_StartMenuAllPrograms,
            Self::Startup => &FOLDERID_Startup,
            Self::SyncManagerFolder => &FOLDERID_SyncManagerFolder,
            Self::SyncResultsFolder => &FOLDERID_SyncResultsFolder,
            Self::SyncSetupFolder => &FOLDERID_SyncSetupFolder,
            Self::System => &FOLDERID_System,
            Self::SystemX86 => &FOLDERID_SystemX86,
            Self::Templates => &FOLDERID_Templates,
            Self::UserPinned => &FOLDERID_UserPinned,
            Self::UserProfiles => &FOLDERID_UserProfiles,
            Self::UserProgramFiles => &FOLDERID_UserProgramFiles,
            Self::UserProgramFilesCommon => &FOLDERID_UserProgramFilesCommon,
            Self::UsersFiles => &FOLDERID_UsersFiles,
            Self::UsersLibraries => &FOLDERID_UsersLibraries,
            Self::Videos => &FOLDERID_Videos,
            Self::VideosLibrary => &FOLDERID_VideosLibrary,
            Self::Windows => &FOLDERID_Windows,
        }
    }
}

/// Retrieve the full path of a known folder identified by the folder's
/// [`KNOWNFOLDERID`].
///
/// A safe wrapper around the [`SHGetKnownFolderPath`] Win32 API function on
/// Windows.
///
/// See [`KnownFolder`] for the types of known folders this function can
/// retrieve.
///
/// # Errors
///
/// If an error occurs when calling the underlying Windows APIs or the given
/// Known Folder ID is not present on the system (for example, if the ID was
/// introduced in a newer OS version), [`None`] is returned.
///
/// # Examples
///
/// ```
/// use known_folders::{get_known_folder_path, KnownFolder};
///
/// let profile_dir = get_known_folder_path(KnownFolder::Profile);
/// ```
///
/// [`KNOWNFOLDERID`]: KnownFolder
/// [`SHGetKnownFolderPath`]: https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath
pub fn get_known_folder_path(known_folder: KnownFolder) -> Option<PathBuf> {
    // This guard ensures `CoTaskMemFree` is always called after invoking
    // `SHGetKnownFolderPath`, which is required regardless of the return
    // value.
    //
    // See `ppszPath` out parameter description:
    //
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters
    let mut guard = guard::FreeGuard::default();

    // Upstream docs:
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath
    //
    // `SHGetKnownFolderPath` replaces `SHGetFolderPathW` as of Windows Vista:
    //
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetfolderpathw
    //
    // SAFETY: this invocation meets the preconditions defined in the API
    // documentation:
    //
    // - `rfid` is a reference to a known folder ID, provided by `windows-sys`.
    // - `dwFlags` can be `0` per the documentation, we have no special retrieval
    //   requirements, so use the default defined in `windows-sys`.
    //   The `KNOWN_FOLDER_FLAG` enum is documented here:
    //   https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/ne-shlobj_core-known_folder_flag
    // - `hToken` is "an access token that represents a particular user. If this
    //   parameter is `NULL`, which is the most common usage, the function
    //   requests the known folder for the current user. We want the known folder
    //   for the current user, so use `HANDLE::default()`.
    // - `ppszPath` is an out parameter and should be a NULL pointer to a PWSTR.
    //
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#parameters
    match unsafe {
        SHGetKnownFolderPath(
            known_folder.to_guid(),
            KF_FLAG_DEFAULT,
            HANDLE::default(),
            guard.as_out_ppszPath(),
        )
    } {
        S_OK => {
            let path_ptr = guard.as_pwstr();

            // SAFETY: on success, the out pointer is guaranteed to be a valid,
            // NUL-terminated wide string.
            //
            // > When `SHGetKnownFolderPath` returns, contains the address of a
            // > pointer to a null-terminated Unicode string that specifies the
            // > path of the known folder
            let len = unsafe {
                let len = lstrlenW(path_ptr);
                usize::try_from(len).ok()?;
            };

            // SAFETY: `path_ptr` is valid for `len` bytes in a single string
            // allocation, per windows-sys APIs. `lstrlenW` returns `i32` on
            // 64-bit platforms. The `match` below guarantees the size of the
            // allocation is no larger than `isize::MAX`.
            let path = unsafe {
                match isize::try_from(len) {
                    Ok(len) if len < 0 => return None,
                    Ok(len) if len.checked_mul(size_of::<u16>() as isize).is_some() => {}
                    Ok(_) | Err(_) => return None,
                };

                slice::from_raw_parts(path_ptr, len)
            };

            let os_str = OsString::from_wide(path);
            Some(os_str)
        }
        E_FAIL | E_INVALIDARG => {
            // Expected return codes. See:
            //
            // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath#return-value
            None
        }
        _ => {
            // Unexpected return code.
            None
        }
    }
}
