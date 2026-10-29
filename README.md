## Przeglądarka PPM

Przeglądarka obrazów medycznych `.dcm` wraz z ich metadanymi.

## Stack
| Biblioteka | Zastosowanie |
|---|---|
| [`eframe` / `egui`](https://docs.rs/egui) | GUI |
| [`dicom-object`](https://docs.rs/dicom-object) | odczyt pliku & metadanych DICOM |
| [`dicom-dictionary-std`](https://docs.rs/dicom-dictionary-std) | standardowy słownik nazw DICOM |
| [`dicom-pixeldata`](https://docs.rs/dicom-pixeldata) | przetwarzanie danych pikselowych |
| [`rfd`](https://docs.rs/rfd) | dialog do wyboru pliku |

#### Wymagania funkcjonalne
- Użytkownik wskazuje plik DICOM (.dcm) z dysku.
- Program poprawnie wczytuje dane obrazu.
- Obraz wyświetlany jest w oknie. Użytkownik może powiększać obraz oraz przesuwać. Przy dużym powiększeniu gdy widać pojedyncze piksele, to na pikselach pojawia się napis z wartością piksela (np. 128, 128, 128 w przypadku RGB, albo np. 256 w przypadku obrazu 8-bit odcienie szarości).
- W przypadku wielowarstwowych badań (np. tomografia) można przeglądać poszczególne warstwy (np. z menu albo klawiszami ↑ ↓).
- Program w osobnym panelu pokazuje metadane z nagłówka DICOM, np. PatientName, PatientID, StudyDate, Modality, Rows, Columns, PixelSpacing, SliceThickness, etc. 
    - Uwaga: nie wszystkie dane mogą być w pliku dostępne, idealnie jakby program wyświetlał wszystkie niepuste metadane.
- Opcjonalnie na ocenę 5.0: obsługa folderu z wieloma plikami DICOM (np. całe badanie CT)

## Uruchamianie
### Rust
Do uruchomienia potrzebne są Rust i jego package manager - Cargo [dostępne pod tym linkiem.](https://rust-lang.org/tools/install/)

... albo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Uruchamianie

```bash
cargo run --release
```

