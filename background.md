# Background task (optional)

I suggest using `supervisord` (so install it first).

1. Edit [supervisord config](scripts/spotify_jbl_remote_supervisord.conf): change **user=root** to username of the user you are planning to run Spotify Desktop from. That means that `spotify-jbl-remote` and Spotify Desktop have to be run by the _same user_.

2. Carefully add/edit these lines (`/etc/supervisord.conf`):
   ```
   [include]
   files = /etc/supervisor.d/*.conf
   ```

   Please note that you should _append_ `/etc/supervisor.d/*.conf` entry. Don't overwrite other include paths.

3. Then (you should probably run these with `sudo`):
   ```bash
   $ mkdir -p /etc/supervisor.d/
   $ cp scripts/spotify_jbl_remote_supervisord.conf /etc/supervisor.d/spotify_jbl_remote.conf
   $ supervisorctl reload
   ```
