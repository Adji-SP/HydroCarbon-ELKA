# Dokumentasi API Backend PHP untuk Pemula

## Pengenalan
API ini adalah backend sederhana yang dibuat dengan PHP untuk mengelola data di database MySQL. API ini menyediakan 2 fungsi utama:
1. **Menyimpan data** ke database (POST)
2. **Mengambil data** dari database (GET)

## URL Dasar API
```
https://yourserver.com/api
```
*Ganti `yourserver.com` dengan alamat server Anda*

---

## 1. MENYIMPAN DATA (POST)

### Endpoint
```
POST /api/maui-data
```

### Fungsi
Menyimpan data baru ke dalam tabel database yang ditentukan.

### Format Data yang Dikirim
Data harus dikirim dalam format JSON dengan struktur berikut:

```json
{
  "tableName": "nama_tabel",
  "records": [
    {
      "field1": "value1",
      "field2": "value2",
      "field3": "value3"
    }
  ]
}
```

### Contoh Penggunaan
Untuk menyimpan data pengguna:

```json
{
  "tableName": "users",
  "records": [
    {
      "nama": "John Doe",
      "email": "john@example.com",
      "umur": 25,
      "tanggal_dibuat": "2025-06-17 10:30:00"
    }
  ]
}
```

### Contoh Response Sukses
```json
{
  "success": true,
  "message": "Successfully inserted 1 records into 'users'.",
  "insertedIds": [15]
}
```

### Contoh Response Error
```json
{
  "success": false,
  "error": "A 'tableName' string is required in the request body."
}
```

### Cara Kerja Backend
1. API menerima data JSON dari aplikasi MAUI
2. Memvalidasi apakah `tableName` dan `records` ada
3. Mengecek apakah struktur data benar
4. Menyimpan setiap record ke database
5. Mengembalikan response sukses atau error

---

## 2. MENGAMBIL DATA (GET)

### Endpoint
```
GET /api/maui-get/{nama_tabel}
```

### Fungsi
Mengambil data dari tabel database yang ditentukan.

### Format URL

#### Mengambil semua data:
```
GET /api/maui-get/users
```

#### Mengambil data dengan filter:
```
GET /api/maui-get/users?filters[nama]=John&filters[umur]=25
```

#### Mengambil data dengan pengurutan:
```
GET /api/maui-get/users?orderBy[column]=nama&orderBy[direction]=ASC
```

#### Membatasi jumlah data:
```
GET /api/maui-get/users?limit=10
```

### Contoh Response Sukses
```json
{
  "success": true,
  "message": "Successfully retrieved 2 records from 'users'.",
  "data": [
    {
      "id": "1",
      "nama": "John Doe",
      "email": "john@example.com",
      "umur": "25",
      "tanggal_dibuat": "2025-06-17 10:30:00"
    },
    {
      "id": "2",
      "nama": "Jane Smith",
      "email": "jane@example.com",
      "umur": "30",
      "tanggal_dibuat": "2025-06-17 11:15:00"
    }
  ],
  "count": 2
}
```

### Contoh Response Error
```json
{
  "success": false,
  "error": "Table name is required in URL."
}
```

### Cara Kerja Backend
1. API menerima request GET dari aplikasi MAUI
2. Mengambil nama tabel dari URL
3. Memproses filter jika ada
4. Mengambil data dari database
5. Mengembalikan data dalam format JSON

---

## Struktur Database

### Tabel `users` (contoh)
```sql
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nama VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    umur INT,
    tanggal_dibuat DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

## Error Codes dan Penanganannya

### HTTP Status Codes
- **200 OK**: Request berhasil
- **201 Created**: Data berhasil dibuat
- **400 Bad Request**: Data yang dikirim tidak valid
- **405 Method Not Allowed**: Method HTTP salah
- **500 Internal Server Error**: Error server

### Contoh Penanganan Error
```json
{
  "success": false,
  "error": "Invalid JSON input."
}
```

---

## Tips untuk Pemula

### 1. Testing API
Gunakan tools seperti Postman atau browser untuk test API:
```
GET https://yourserver.com/api/health
```

### 2. Debugging
Jika ada error, cek:
- Apakah URL sudah benar?
- Apakah format JSON sudah benar?
- Apakah nama tabel sudah benar?
- Apakah koneksi database berjalan?

### 3. Keamanan
- Jangan expose kredensial database
- Gunakan HTTPS untuk production
- Validasi semua input data

### 4. Konfigurasi
Pastikan file konfigurasi database sudah benar:
```php
$config = [
    'host' => 'localhost',
    'user' => 'your_username',
    'password' => 'your_password',
    'database' => 'your_database'
];
```

---

## Contoh Implementasi di C# MAUI

### Mengirim Data
```csharp
var data = new {
    tableName = "users",
    records = new[] {
        new {
            nama = "John",
            email = "john@email.com",
            umur = 25
        }
    }
};

string json = JsonSerializer.Serialize(data);
var content = new StringContent(json, Encoding.UTF8, "application/json");
var response = await httpClient.PostAsync(url, content);
```

### Mengambil Data
```csharp
string url = "https://yourserver.com/api/maui-get/users";
var response = await httpClient.GetAsync(url);
string responseText = await response.Content.ReadAsStringAsync();
```

---

## Kesimpulan

API ini menyediakan cara sederhana untuk:
1. **POST /api/maui-data**: Menyimpan data ke database
2. **GET /api/maui-get/{table}**: Mengambil data dari database

Kedua endpoint ini cukup untuk aplikasi CRUD sederhana. Pastikan selalu validasi data dan handle error dengan baik!
