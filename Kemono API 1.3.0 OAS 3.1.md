# Kemono API 1.3.0 OAS 3.1

- Version: `1.3.0 | OAS 3.1`

## Servers
- `https://kemono.cr/api`
- `https://coomer.su/api`

## Posts
- Tag Description: Version one

### `GET` `/v1/creators.txt`
- Summary: List All Creators
- Description: List all creators with details. I blame DDG for .txt.
- Parameters: No parameters
- Responses:
  - `200`: List of all creators

### `GET` `/v1/posts`
- Summary: List recent posts
- Description: List of recently imported posts
- Parameters:
  - `q` (string) (query): Search query
  - `o` (integer) (query): Result offset, stepping of 50 is enforced
  - `tag` (array<string>) (query): A list of tags to filter by
- Responses:
  - `200`: List of recently added posts

### `GET` `/v1/{service}/user/{creator_id}/announcements`
- Summary: Get creator announcements
- Parameters:
  - `service *` (string) (path): The service name
  - `creator_id *` (string) (path): The creator's ID
- Responses:
  - `200`: Successful response
  - `404`: Artist not found

### `GET` `/v1/{service}/user/{creator_id}/fancards`
- Summary: Get fancards by creator, fanbox only
- Parameters:
  - `service *` (string) (path): The service name, has to be "fanbox"
  - `creator_id *` (string) (path): The creator's ID
- Responses:
  - `200`: Successful response
  - `404`: Artist not found

### `GET` `/v1/{service}/user/{creator_id}/post/{post_id}`
- Summary: Get a specific post
- Parameters:
  - `service *` (string) (path): The service name
  - `creator_id *` (string) (path): The creator's ID
  - `post_id *` (string) (path): The post ID
- Responses:
  - `200`: Successful response
  - `404`: Post not found

### `GET` `/v1/{service}/user/{creator_id}/post/{post_id}/revisions`
- Summary: List a Post's Revisions
- Description: List revisions of a specific post by service, creator_id, and post_id
- Parameters:
  - `service *` (string) (path): The service where the post is located
  - `creator_id *` (string) (path): The ID of the creator
  - `post_id *` (string) (path): The ID of the post
- Responses:
  - `200`: List of post revisions
  - `404`: Post not found

## Creators

### `GET` `/v1/{service}/user/{creator_id}/profile`
- Summary: Get a creator
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): The ID of the creator
- Responses:
  - `200`: Creator details retrieved successfully
  - `404`: The creator could not be found

### `GET` `/v1/{service}/user/{creator_id}/links`
- Summary: Get a creator's linked accounts
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): The ID of the creator
- Responses:
  - `200`: Linked accounts retrieved successfully
  - `404`: The creator could not be found

### `GET` `/v1/{service}/user/{creator_id}/tags`

## Comments

### `GET` `/v1/{service}/user/{creator_id}/post/{post_id}/comments`
- Summary: List a post's comments
- Description: List comments for a specific post by service, creator_id, and post_id.
- Parameters:
  - `service *` (string) (path): The post's service.
  - `creator_id *` (string) (path): The service ID of the post's creator.
  - `post_id *` (string) (path): The service ID of the post.
- Responses:
  - `200`: List of post comments.
  - `404`: No comments found.

## Post Flagging
- Tag Description: Flag post for re-import

### `POST` `/v1/{service}/user/{creator_id}/post/{post}/flag`
- Summary: Flag a post
- Parameters:
  - `service *` (string) (path)
  - `creator_id *` (string) (path)
  - `post *` (string) (path)
- Responses:
  - `201`: Flagged successfully
  - `409`: Already flagged

### `GET` `/v1/{service}/user/{creator_id}/post/{post}/flag`
- Summary: Check if a Post is flagged
- Description: Check if a Post is flagged
- Parameters:
  - `service *` (string) (path): The service where the post is located
  - `creator_id *` (string) (path): The creator of the post
  - `post *` (string) (path): The ID of the post to flag
- Responses:
  - `200`: The post is flagged
  - `404`: The post has no flag

## Discord

### `GET` `/v1/discord/channel/{channel_id}`
- Summary: Get Discord channel posts by offset
- Parameters:
  - `channel_id *` (string) (path): ID of the Discord channel
  - `o` (integer) (query): Result offset, stepping of 150 is enforced
- Responses:
  - `200`: Discord channel found
  - `404`: Discord channel not found

### `GET` `/v1/discord/channel/lookup/{discord_server}`
- Summary: Lookup Discord channels
- Parameters:
  - `discord_server *` (string) (path): Discord Server ID
- Responses:
  - `200`: Discord channels found
  - `404`: Discord server not found

## Favorites

### `GET` `/v1/account/favorites`
- Summary: List Account Favorites
- Description: List account favorites (posts or creators) for the authenticated user (cookie session)
- Parameters:
  - `type` (string) (query): Type of favorites to list (post or creator (artist) )
- Responses:
  - `200`: List of account favorites
  - `401`: Unauthorized Access

### `POST` `/v1/favorites/post/{service}/{creator_id}/{post_id}`
- Summary: Add Favorite Post
- Description: Add a post to the user's favorite posts
- Parameters:
  - `service *` (string) (path): Service of the post
  - `creator_id *` (string) (path): The ID of the creator
  - `post_id *` (string) (path): The ID of the post
- Responses:
  - `200`: Favorite post added successfully
  - `302`: Redirect to login if not authenticated
  - `401`: Unauthorized Access

### `DELETE` `/v1/favorites/post/{service}/{creator_id}/{post_id}`
- Summary: Remove Favorite Post
- Description: Remove a post from the user's favorite posts
- Parameters:
  - `service *` (string) (path): The service where the post is located
  - `creator_id *` (string) (path): The ID of the creator
  - `post_id *` (string) (path): The ID of the post
- Responses:
  - `200`: Unfavorite post removed successfully
  - `302`: Redirect to login if not authenticated
  - `401`: Unauthorized Access

### `POST` `/v1/favorites/creator/{service}/{creator_id}`
- Summary: Add Favorite creator
- Description: Add an creator to the user's favorite creators
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): The ID of the creator
- Responses:
  - `200`: Favorite creator added successfully
  - `302`: Redirect to login if not authenticated
  - `401`: Unauthorized Access

### `DELETE` `/v1/favorites/creator/{service}/{creator_id}`
- Summary: Remove Favorite Creator
- Description: Remove an creator from the user's favorite creators
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): The ID of the creator
- Responses:
  - `200`: Favorite creator removed successfully
  - `302`: Redirect to login if not authenticated
  - `401`: Unauthorized Access

## File Search

### `GET` `/v1/search_hash/{file_hash}`
- Summary: Lookup file by hash
- Parameters:
  - `file_hash *` (string ($hex)) (path): SHA-2 / SHA-256
- Responses:
  - `200`: File found
  - `404`: File not found

## Misc

### `GET` `/v1/app_version`
- Summary: Git Commit Hash
- Description: Show current App commit hash
- Parameters: No parameters
- Responses:
  - `200`: Commit Hash

## default

### `GET` `/v2/file/{file_hash}`
- Description: Overview of the file.
- Parameters:
  - `file_hash *` (string) (path): Hash of the file.
- Responses:
  - `200`: Successfully retrieved file details.
  - `400`: There are errors in parameters.
  - `404`: File does not exist.

### `PATCH` `/v2/file/{file_hash}`
- Description: Add password to a file if needed
- Parameters:
  - `file_hash *` (string) (path): Hash of the file.
- Responses:
  - `200`: Successfully added a correct password.
  - `400`: There are errors in parameters or the body.

### `PUT` `/v2/account/flags/post`
- Description: Flag the post for reimport.
- Parameters: No parameters
- Responses:
  - `201`: Successfully flagged the post.
  - `400`: Request body has errors.
  - `404`: Post doesn't exist.
  - `409`: Post is already flagged.

### `GET` `/v2/account/administrator/accounts`
- Description: Get account count.
- Parameters:
  - `name` (string) (query): Filter by name
  - `role` (string) (query): Filter by role
- Responses:
  - `200`: Successfully counted accounts.
  - `401`: User not logged in.
  - `404`: User is not administrator.

### `GET` `/v2/account/administrator/accounts/{page}`
- Description: Get accounts at page.
- Parameters:
  - `page *` (integer) (path): Page of the collection.
  - `name` (string) (query): Filter by name
  - `role` (string) (query): Filter by role
- Responses:
  - `200`: Successfully gotten accounts at page.
  - `400`: There are errors in parameters.
  - `401`: User not logged in.
  - `404`: User is not administrator.

### `GET` `/v2/account/administrator/account/{account_id}`
- Description: Overview of target account.
- Parameters:
  - `account_id *` (integer) (path): ID of the account.
- Responses:
  - `200`: Successfully retrieved target account details.
  - `400`: There are errors in parameters.
  - `401`: User not logged in.
  - `404`: User is not administrator or account doesn't exist.

### `PATCH` `/v2/account/administrator/account/{account_id}`

### `GET` `/v1/posts/random`
- Description: Get a random post
- Parameters: No parameters
- Responses:
  - `200`: A random post.
  - `404`: Not random psot found.

### `GET` `/v1/posts/popular`
- Description: Get popular posts
- Parameters:
  - `date *` (string) (query): Base date of the list
  - `period *` (any) (query): Period scale of the list
  - `o` (integer) (query): Result offset, stepping of 50 is enforced
- Responses:
  - `200`: A list of popular posts.

### `GET` `/v1/posts/tags`
- Description: Get tags
- Parameters: No parameters
- Responses:
  - `200`: A list of post tags.

### `GET` `/v1/{service}/post/{post_id}`
- Description: Get a post by ID
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `post_id *` (string) (path): ID of the post
- Responses:
  - `200`: Post data.
  - `404`: No post found

### `DELETE` `/v1/{service}/user/{creator_id}/links`
- Description: Remove artist from linked accounts. Requires admin privilegies.
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): ID of the creator
- Responses:
  - `204`: Artist's link was successfuly removed.
  - `404`: Insufficient privilegies.

### `GET` `/v1/{service}/user/{creator_id}/links/new`
- Description: Add links to the artist
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): ID of the creator
- Responses:
  - `200`: The data for the new link.

### `POST` `/v1/{service}/user/{creator_id}/links/new`
- Description: Add links to the artist
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): ID of the creator
- Responses:
  - `200`: The link request added to moderation queue.
  - `400`: Failed to added the new link due to input errors.

### `GET` `/v1/{service}/user/{creator_id}/shares`
- Description: Shares of the artist
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): ID of the creator
  - `o` (integer) (query): Result offset, stepping of 50 is enforced
- Responses:
  - `200`: Found the shares for the artist

### `GET` `/v1/{service}/user/{creator_id}/dms`
- Description: Direct messages of profile
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): ID of the creator
- Responses:
  - `200`: Found direct messages for the profile

### `GET` `/v1/{service}/user/{creator_id}/posts`
- Description: A duct-tape endpoint which also returns count for pagination component.
- Parameters:
  - `service *` (string) (path): The service name
  - `creator_id *` (string) (path): The profiles's ID
  - `tag` (array<any>) (query): A list of post tags
- Responses:
  - `200`: Found posts of the profile

### `GET` `/v1/{service}/user/{creator_id}/post/{post_id}/revision/{revision_id}`
- Description: Get revision of a post
- Parameters:
  - `service *` (string) (path): The service where the creator is located
  - `creator_id *` (string) (path): ID of the creator
  - `post_id *` (string) (path): ID of the post
  - `revision_id *` (string) (path): ID of the revision
- Responses:
  - `200`: A revision of the post.
  - `404`: Failed to find the revision of the post.

### `POST` `/v1/authentication/register`
- Description: Register an account
- Parameters: No parameters
- Responses:
  - `200`: Successfully registered.
  - `400`: Failed to register due to user errors.

### `POST` `/v1/authentication/login`
- Description: Sign in to account
- Parameters: No parameters
- Responses:
  - `200`: Succefully logged in.
  - `400`: Failed to log in due to user errors.
  - `409`: Already logged in.

### `POST` `/v1/authentication/logout`
- Description: Logout from account
- Parameters: No parameters
- Responses:
  - `200`: Succefuuly logged out from account.

### `GET` `/v1/account`
- Description: Get account data
- Parameters: No parameters
- Responses:
  - `200`: Account data.

### `POST` `/v1/account/change_password`
- Description: Change account password
- Parameters: No parameters
- Responses:
  - `200`: Successfully changed account password.

### `GET` `/v1/account/notifications`
- Description: Get account notifications
- Parameters: No parameters
- Responses:
  - `200`: A list of account notifications.

### `GET` `/v1/account/keys`
- Description: Get account autoimport keys
- Parameters: No parameters
- Responses:
  - `200`: A list of account keys.

### `POST` `/v1/account/keys`
- Description: Revoke account autoimport keys
- Parameters: No parameters
- Responses:
  - `200`: Account import keys revoked.

### `GET` `/v1/account/posts/upload`
- Description: Upload posts.
- Parameters: No parameters
- Responses:
  - `200`: Upload posts maybe???

### `GET` `/v1/account/review_dms`
- Description: Get DMs for review.
- Parameters:
  - `status` (any) (query): Status of the DM.
- Responses:
  - `200`: A list of unapproved DMs.

### `POST` `/v1/account/review_dms`
- Description: Approve DMs.
- Parameters: No parameters
- Responses:
  - `200`: Approved DMs.

### `GET` `/v1/account/moderator/tasks/creator_links`
- Description: Get a list of pending artist link requests
- Parameters: No parameters
- Responses:
  - `200`: A list of pending artist link requests.

### `POST` `/v1/account/moderator/creator_link_requests/{request_id}/approve`
- Description: Approve a new artist link.
- Parameters: No parameters
- Responses:
  - `200`: Successfully approved a new artist link.

### `POST` `/v1/account/moderator/creator_link_requests/{request_id}/reject`
- Description: Reject a new artist link.
- Parameters: No parameters
- Responses:
  - `200`: Successfully rejected a new artist link.

### `GET` `/v1/artists/random`
- Description: Get a random artist
- Parameters: No parameters
- Responses:
  - `200`: A random artist.
  - `404`: No random artst exists.

### `GET` `/v1/shares`
- Description: Get a list of shares
- Parameters:
  - `o` (integer) (query): List's offset
- Responses:
  - `200`: A list of shares.

### `GET` `/v1/share/{share_id}`
- Description: Get details of the share.
- Parameters:
  - `share_id *` (string) (path): ID of the share.
- Responses:
  - `200`: Details of the share.

### `GET` `/v1/dms`
- Description: Get a list of DMs.
- Parameters:
  - `o` (integer) (query): List's offset
  - `q` (string) (query): Search query
- Responses:
  - `200`: A list of DMs.

### `GET` `/v1/has_pending_dms`
- Description: Check if there are pending DMs.
- Parameters: No parameters
- Responses:
  - `200`: There are pending DMs.

### `POST` `/v1/importer/submit`
- Description: Create a site import
- Parameters: No parameters
- Responses:
  - `200`: Succesfully added new import

### `GET` `/v1/importer/logs/{import_id}`
- Parameters: No parameters
- Responses:
  - `200`: Get import logs
