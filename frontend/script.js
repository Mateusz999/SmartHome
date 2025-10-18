fetch("http://192.168.1.200:5000/api/data") // ← wpisz IP Raspberry Pi
  .then(res => res.json())
  .then(data => {
    document.getElementById("temp").innerText = data.temperature;
    document.getElementById("hum").innerText = data.humidity;
  })
  .catch(err => {
    console.error("Błąd połączenia z API:", err);
  });
