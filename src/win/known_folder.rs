// src/win/known_folder.rs
//
// Copyright (c) 2023 Ryan Lopopolo <rjl@hyperbo.la>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
// <http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT>
// or <http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use windows_sys::core::GUID;
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

/// GUIDs that identify standard folders registered with the system as
/// [Known Folders].
///
/// These folders are installed with Windows Vista and later operating systems,
/// and a computer will have only folders appropriate to it installed.
///
/// For details on the **KNOWNFOLDERID** constants this enum represents, please
/// refer to the [upstream documentation].
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
    /// Known Folder ID `FOLDERID_AccountPictures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AccountPictures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AccountPictures>
    AccountPictures,
    /// Known Folder ID `FOLDERID_AddNewPrograms`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AddNewPrograms`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AddNewPrograms>
    AddNewPrograms,
    /// Known Folder ID `FOLDERID_AdminTools`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AdminTools`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AdminTools>
    AdminTools,
    /// Known Folder ID `FOLDERID_AllAppMods`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AllAppMods`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AllAppMods>
    AllAppMods,
    /// Known Folder ID `FOLDERID_AppCaptures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AppCaptures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppCaptures>
    AppCaptures,
    /// Known Folder ID `FOLDERID_AppDataDesktop`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AppDataDesktop`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataDesktop>
    AppDataDesktop,
    /// Known Folder ID `FOLDERID_AppDataDocuments`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AppDataDocuments`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataDocuments>
    AppDataDocuments,
    /// Known Folder ID `FOLDERID_AppDataFavorites`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AppDataFavorites`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataFavorites>
    AppDataFavorites,
    /// Known Folder ID `FOLDERID_AppDataProgramData`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AppDataProgramData`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataProgramData>
    AppDataProgramData,
    /// Known Folder ID `FOLDERID_AppUpdates`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AppUpdates`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppUpdates>
    AppUpdates,
    /// Known Folder ID `FOLDERID_ApplicationShortcuts`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ApplicationShortcuts`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ApplicationShortcuts>
    ApplicationShortcuts,
    /// Known Folder ID `FOLDERID_AppsFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_AppsFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppsFolder>
    AppsFolder,
    /// Known Folder ID `FOLDERID_CDBurning`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CDBurning`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CDBurning>
    CDBurning,
    /// Known Folder ID `FOLDERID_CameraRoll`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CameraRoll`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CameraRoll>
    CameraRoll,
    /// Known Folder ID `FOLDERID_CameraRollLibrary`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CameraRollLibrary`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CameraRollLibrary>
    CameraRollLibrary,
    /// Known Folder ID `FOLDERID_ChangeRemovePrograms`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ChangeRemovePrograms`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ChangeRemovePrograms>
    ChangeRemovePrograms,
    /// Known Folder ID `FOLDERID_CommonAdminTools`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CommonAdminTools`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonAdminTools>
    CommonAdminTools,
    /// Known Folder ID `FOLDERID_CommonOEMLinks`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CommonOEMLinks`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonOEMLinks>
    CommonOEMLinks,
    /// Known Folder ID `FOLDERID_CommonPrograms`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CommonPrograms`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonPrograms>
    CommonPrograms,
    /// Known Folder ID `FOLDERID_CommonStartMenu`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CommonStartMenu`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonStartMenu>
    CommonStartMenu,
    /// Known Folder ID `FOLDERID_CommonStartMenuPlaces`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CommonStartMenuPlaces`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonStartMenuPlaces>
    CommonStartMenuPlaces,
    /// Known Folder ID `FOLDERID_CommonStartup`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CommonStartup`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonStartup>
    CommonStartup,
    /// Known Folder ID `FOLDERID_CommonTemplates`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CommonTemplates`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CommonTemplates>
    CommonTemplates,
    /// Known Folder ID `FOLDERID_ComputerFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ComputerFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ComputerFolder>
    ComputerFolder,
    /// Known Folder ID `FOLDERID_ConflictFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ConflictFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ConflictFolder>
    ConflictFolder,
    /// Known Folder ID `FOLDERID_ConnectionsFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ConnectionsFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ConnectionsFolder>
    ConnectionsFolder,
    /// Known Folder ID `FOLDERID_Contacts`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Contacts`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Contacts>
    Contacts,
    /// Known Folder ID `FOLDERID_ControlPanelFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ControlPanelFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ControlPanelFolder>
    ControlPanelFolder,
    /// Known Folder ID `FOLDERID_Cookies`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Cookies`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Cookies>
    Cookies,
    /// Known Folder ID `FOLDERID_CurrentAppMods`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_CurrentAppMods`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_CurrentAppMods>
    CurrentAppMods,
    /// Known Folder ID `FOLDERID_Desktop`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Desktop`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Desktop>
    Desktop,
    /// Known Folder ID `FOLDERID_DevelopmentFiles`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_DevelopmentFiles`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_DevelopmentFiles>
    DevelopmentFiles,
    /// Known Folder ID `FOLDERID_Device`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Device`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Device>
    Device,
    /// Known Folder ID `FOLDERID_DeviceMetadataStore`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_DeviceMetadataStore`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_DeviceMetadataStore>
    DeviceMetadataStore,
    /// Known Folder ID `FOLDERID_Documents`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Documents`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Documents>
    Documents,
    /// Known Folder ID `FOLDERID_DocumentsLibrary`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_DocumentsLibrary`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_DocumentsLibrary>
    DocumentsLibrary,
    /// Known Folder ID `FOLDERID_Downloads`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Downloads`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Downloads>
    Downloads,
    /// Known Folder ID `FOLDERID_Favorites`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Favorites`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Favorites>
    Favorites,
    /// Known Folder ID `FOLDERID_Fonts`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Fonts`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Fonts>
    Fonts,
    /// Known Folder ID `FOLDERID_GameTasks`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_GameTasks`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_GameTasks>
    GameTasks,
    /// Known Folder ID `FOLDERID_Games`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Games`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Games>
    Games,
    /// Known Folder ID `FOLDERID_History`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_History`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_History>
    History,
    /// Known Folder ID `FOLDERID_HomeGroup`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_HomeGroup`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_HomeGroup>
    HomeGroup,
    /// Known Folder ID `FOLDERID_HomeGroupCurrentUser`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_HomeGroupCurrentUser`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_HomeGroupCurrentUser>
    HomeGroupCurrentUser,
    /// Known Folder ID `FOLDERID_ImplicitAppShortcuts`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ImplicitAppShortcuts`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ImplicitAppShortcuts>
    ImplicitAppShortcuts,
    /// Known Folder ID `FOLDERID_InternetCache`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_InternetCache`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_InternetCache>
    InternetCache,
    /// Known Folder ID `FOLDERID_InternetFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_InternetFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_InternetFolder>
    InternetFolder,
    /// Known Folder ID `FOLDERID_Libraries`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Libraries`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Libraries>
    Libraries,
    /// Known Folder ID `FOLDERID_Links`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Links`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Links>
    Links,
    /// Known Folder ID `FOLDERID_LocalAppData`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalAppData`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalAppData>
    LocalAppData,
    /// Known Folder ID `FOLDERID_LocalAppDataLow`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalAppDataLow`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalAppDataLow>
    LocalAppDataLow,
    /// Known Folder ID `FOLDERID_LocalDocuments`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalDocuments`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalDocuments>
    LocalDocuments,
    /// Known Folder ID `FOLDERID_LocalDownloads`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalDownloads`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalDownloads>
    LocalDownloads,
    /// Known Folder ID `FOLDERID_LocalMusic`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalMusic`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalMusic>
    LocalMusic,
    /// Known Folder ID `FOLDERID_LocalPictures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalPictures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalPictures>
    LocalPictures,
    /// Known Folder ID `FOLDERID_LocalStorage`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalStorage`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalStorage>
    LocalStorage,
    /// Known Folder ID `FOLDERID_LocalVideos`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalVideos`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalVideos>
    LocalVideos,
    /// Known Folder ID `FOLDERID_LocalizedResourcesDir`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_LocalizedResourcesDir`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalizedResourcesDir>
    LocalizedResourcesDir,
    /// Known Folder ID `FOLDERID_Music`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Music`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Music>
    Music,
    /// Known Folder ID `FOLDERID_MusicLibrary`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_MusicLibrary`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_MusicLibrary>
    MusicLibrary,
    /// Known Folder ID `FOLDERID_NetHood`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_NetHood`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_NetHood>
    NetHood,
    /// Known Folder ID `FOLDERID_NetworkFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_NetworkFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_NetworkFolder>
    NetworkFolder,
    /// Known Folder ID `FOLDERID_Objects3D`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Objects3D`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Objects3D>
    Objects3D,
    /// Known Folder ID `FOLDERID_OneDrive`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_OneDrive`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_OneDrive>
    OneDrive,
    /// Known Folder ID `FOLDERID_OriginalImages`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_OriginalImages`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_OriginalImages>
    OriginalImages,
    /// Known Folder ID `FOLDERID_PhotoAlbums`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PhotoAlbums`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PhotoAlbums>
    PhotoAlbums,
    /// Known Folder ID `FOLDERID_Pictures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Pictures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Pictures>
    Pictures,
    /// Known Folder ID `FOLDERID_PicturesLibrary`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PicturesLibrary`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PicturesLibrary>
    PicturesLibrary,
    /// Known Folder ID `FOLDERID_Playlists`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Playlists`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Playlists>
    Playlists,
    /// Known Folder ID `FOLDERID_PrintHood`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PrintHood`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PrintHood>
    PrintHood,
    /// Known Folder ID `FOLDERID_PrintersFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PrintersFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PrintersFolder>
    PrintersFolder,
    /// Known Folder ID `FOLDERID_Profile`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Profile`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Profile>
    Profile,
    /// Known Folder ID `FOLDERID_ProgramData`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ProgramData`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramData>
    ProgramData,
    /// Known Folder ID `FOLDERID_ProgramFiles`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ProgramFiles`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFiles>
    ProgramFiles,
    /// Known Folder ID `FOLDERID_ProgramFilesCommon`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ProgramFilesCommon`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesCommon>
    ProgramFilesCommon,
    /// Known Folder ID `FOLDERID_ProgramFilesCommonX64`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ProgramFilesCommonX64`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesCommonX64>
    ProgramFilesCommonX64,
    /// Known Folder ID `FOLDERID_ProgramFilesCommonX86`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ProgramFilesCommonX86`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesCommonX86>
    ProgramFilesCommonX86,
    /// Known Folder ID `FOLDERID_ProgramFilesX64`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ProgramFilesX64`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesX64>
    ProgramFilesX64,
    /// Known Folder ID `FOLDERID_ProgramFilesX86`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ProgramFilesX86`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ProgramFilesX86>
    ProgramFilesX86,
    /// Known Folder ID `FOLDERID_Programs`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Programs`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Programs>
    Programs,
    /// Known Folder ID `FOLDERID_Public`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Public`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Public>
    Public,
    /// Known Folder ID `FOLDERID_PublicDesktop`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicDesktop`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicDesktop>
    PublicDesktop,
    /// Known Folder ID `FOLDERID_PublicDocuments`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicDocuments`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicDocuments>
    PublicDocuments,
    /// Known Folder ID `FOLDERID_PublicDownloads`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicDownloads`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicDownloads>
    PublicDownloads,
    /// Known Folder ID `FOLDERID_PublicGameTasks`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicGameTasks`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicGameTasks>
    PublicGameTasks,
    /// Known Folder ID `FOLDERID_PublicLibraries`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicLibraries`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicLibraries>
    PublicLibraries,
    /// Known Folder ID `FOLDERID_PublicMusic`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicMusic`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicMusic>
    PublicMusic,
    /// Known Folder ID `FOLDERID_PublicPictures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicPictures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicPictures>
    PublicPictures,
    /// Known Folder ID `FOLDERID_PublicRingtones`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicRingtones`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicRingtones>
    PublicRingtones,
    /// Known Folder ID `FOLDERID_PublicUserTiles`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicUserTiles`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicUserTiles>
    PublicUserTiles,
    /// Known Folder ID `FOLDERID_PublicVideos`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_PublicVideos`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_PublicVideos>
    PublicVideos,
    /// Known Folder ID `FOLDERID_QuickLaunch`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_QuickLaunch`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_QuickLaunch>
    QuickLaunch,
    /// Known Folder ID `FOLDERID_Recent`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Recent`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Recent>
    Recent,
    /// Known Folder ID `FOLDERID_RecordedCalls`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_RecordedCalls`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RecordedCalls>
    RecordedCalls,
    /// Known Folder ID `FOLDERID_RecordedTVLibrary`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_RecordedTVLibrary`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RecordedTVLibrary>
    RecordedTVLibrary,
    /// Known Folder ID `FOLDERID_RecycleBinFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_RecycleBinFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RecycleBinFolder>
    RecycleBinFolder,
    /// Known Folder ID `FOLDERID_ResourceDir`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_ResourceDir`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_ResourceDir>
    ResourceDir,
    /// Known Folder ID `FOLDERID_RetailDemo`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_RetailDemo`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RetailDemo>
    RetailDemo,
    /// Known Folder ID `FOLDERID_Ringtones`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Ringtones`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Ringtones>
    Ringtones,
    /// Known Folder ID `FOLDERID_RoamedTileImages`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_RoamedTileImages`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamedTileImages>
    RoamedTileImages,
    /// Known Folder ID `FOLDERID_RoamingAppData`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_RoamingAppData`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamingAppData>
    RoamingAppData,
    /// Known Folder ID `FOLDERID_RoamingTiles`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_RoamingTiles`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamingTiles>
    RoamingTiles,
    /// Known Folder ID `FOLDERID_SEARCH_CSC`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SEARCH_CSC`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SEARCH_CSC>
    SEARCH_CSC,
    /// Known Folder ID `FOLDERID_SEARCH_MAPI`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SEARCH_MAPI`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SEARCH_MAPI>
    SEARCH_MAPI,
    /// Known Folder ID `FOLDERID_SampleMusic`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SampleMusic`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SampleMusic>
    SampleMusic,
    /// Known Folder ID `FOLDERID_SamplePictures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SamplePictures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SamplePictures>
    SamplePictures,
    /// Known Folder ID `FOLDERID_SamplePlaylists`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SamplePlaylists`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SamplePlaylists>
    SamplePlaylists,
    /// Known Folder ID `FOLDERID_SampleVideos`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SampleVideos`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SampleVideos>
    SampleVideos,
    /// Known Folder ID `FOLDERID_SavedGames`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SavedGames`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedGames>
    SavedGames,
    /// Known Folder ID `FOLDERID_SavedPictures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SavedPictures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedPictures>
    SavedPictures,
    /// Known Folder ID `FOLDERID_SavedPicturesLibrary`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SavedPicturesLibrary`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedPicturesLibrary>
    SavedPicturesLibrary,
    /// Known Folder ID `FOLDERID_SavedSearches`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SavedSearches`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SavedSearches>
    SavedSearches,
    /// Known Folder ID `FOLDERID_Screenshots`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Screenshots`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Screenshots>
    Screenshots,
    /// Known Folder ID `FOLDERID_SearchHistory`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SearchHistory`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SearchHistory>
    SearchHistory,
    /// Known Folder ID `FOLDERID_SearchHome`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SearchHome`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SearchHome>
    SearchHome,
    /// Known Folder ID `FOLDERID_SearchTemplates`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SearchTemplates`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SearchTemplates>
    SearchTemplates,
    /// Known Folder ID `FOLDERID_SendTo`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SendTo`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SendTo>
    SendTo,
    /// Known Folder ID `FOLDERID_SidebarDefaultParts`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SidebarDefaultParts`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SidebarDefaultParts>
    SidebarDefaultParts,
    /// Known Folder ID `FOLDERID_SidebarParts`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SidebarParts`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SidebarParts>
    SidebarParts,
    /// Known Folder ID `FOLDERID_SkyDrive`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SkyDrive`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDrive>
    SkyDrive,
    /// Known Folder ID `FOLDERID_SkyDriveCameraRoll`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SkyDriveCameraRoll`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDriveCameraRoll>
    SkyDriveCameraRoll,
    /// Known Folder ID `FOLDERID_SkyDriveDocuments`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SkyDriveDocuments`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDriveDocuments>
    SkyDriveDocuments,
    /// Known Folder ID `FOLDERID_SkyDriveMusic`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SkyDriveMusic`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDriveMusic>
    SkyDriveMusic,
    /// Known Folder ID `FOLDERID_SkyDrivePictures`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SkyDrivePictures`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SkyDrivePictures>
    SkyDrivePictures,
    /// Known Folder ID `FOLDERID_StartMenu`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_StartMenu`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_StartMenu>
    StartMenu,
    /// Known Folder ID `FOLDERID_StartMenuAllPrograms`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_StartMenuAllPrograms`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_StartMenuAllPrograms>
    StartMenuAllPrograms,
    /// Known Folder ID `FOLDERID_Startup`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Startup`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Startup>
    Startup,
    /// Known Folder ID `FOLDERID_SyncManagerFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SyncManagerFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SyncManagerFolder>
    SyncManagerFolder,
    /// Known Folder ID `FOLDERID_SyncResultsFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SyncResultsFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SyncResultsFolder>
    SyncResultsFolder,
    /// Known Folder ID `FOLDERID_SyncSetupFolder`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SyncSetupFolder`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SyncSetupFolder>
    SyncSetupFolder,
    /// Known Folder ID `FOLDERID_System`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_System`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_System>
    System,
    /// Known Folder ID `FOLDERID_SystemX86`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_SystemX86`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_SystemX86>
    SystemX86,
    /// Known Folder ID `FOLDERID_Templates`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Templates`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Templates>
    Templates,
    /// Known Folder ID `FOLDERID_UserPinned`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_UserPinned`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserPinned>
    UserPinned,
    /// Known Folder ID `FOLDERID_UserProfiles`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_UserProfiles`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserProfiles>
    UserProfiles,
    /// Known Folder ID `FOLDERID_UserProgramFiles`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_UserProgramFiles`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserProgramFiles>
    UserProgramFiles,
    /// Known Folder ID `FOLDERID_UserProgramFilesCommon`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_UserProgramFilesCommon`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UserProgramFilesCommon>
    UserProgramFilesCommon,
    /// Known Folder ID `FOLDERID_UsersFiles`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_UsersFiles`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UsersFiles>
    UsersFiles,
    /// Known Folder ID `FOLDERID_UsersLibraries`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_UsersLibraries`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_UsersLibraries>
    UsersLibraries,
    /// Known Folder ID `FOLDERID_Videos`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Videos`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Videos>
    Videos,
    /// Known Folder ID `FOLDERID_VideosLibrary`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_VideosLibrary`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_VideosLibrary>
    VideosLibrary,
    /// Known Folder ID `FOLDERID_Windows`.
    ///
    /// # Upstream Documentation
    ///
    /// - [`windows_sys::Win32::UI::Shell::FOLDERID_Windows`]
    /// - <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Windows>
    Windows,
}

impl KnownFolder {
    #[must_use]
    pub(crate) const fn to_guid(self) -> &'static GUID {
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
