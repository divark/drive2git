Feature: A Cloud Drive should be capable of queries regarding files and folders.
    Scenario: A folder URL from Google Drive should be converted into a Cloud Folder.
        Given a cloud drive logged in using the credentials found in credentials.json,
        And a cloud drive folder found at the URL https://drive.google.com/drive/folders/1LNLV9EDxvvtPjtDOEOVtVyaA8e7saNc4?usp=drive_link,
        When the folder URL is converted into a cloud folder,
        Then the cloud folder should be called 'Public Samples'.

    Scenario: One file should be found in a Google Drive folder.
        Given a cloud drive logged in using the credentials found in credentials.json,
        And a cloud drive folder found at the URL https://drive.google.com/drive/folders/1LNLV9EDxvvtPjtDOEOVtVyaA8e7saNc4?usp=drive_link,
        When the cloud drive is queried for all files in the given cloud folder,
        Then file 'sample.txt' should be in the list of found files.
