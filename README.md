# filter-bg
Aplikasi web yang dapat mengganti background live video
dengan gambar statis yang dapat diupload.

# Implementasi
Ada dua alternatif untuk projek ini:
1. Klien-Server
Kliennya dibuat dengan Javascript dan berfungsi sebagai
antarmuka. Klien akan mengirimkan video stream melalui koneksi
WebSocket ke server dan server mengirim stream video yang
sudah diproses ke klien yang akan menampilkannya di antarmuka.

Server bisa diimplementasikan dengan Python, C++, NodeJS, atau
(fingers crossed) Rust.

2. Semua diproses di klien
OpenCV punya implementasi Javascript berkat Emscripten.
Tidak seribet yang klien-server, tapi kurang seru (bercanda :v).
Walau begitu, implementasi ini bisa dikerjakan secara berkelompok.
