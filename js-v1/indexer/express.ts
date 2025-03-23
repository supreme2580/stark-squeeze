import express from 'express';

const app = express();
app.use(express.json());

app.post('/', (req, res) => {
    console.log("Received data:", req.body);
    res.sendStatus(200);
});

app.listen(3000, () => console.log("Webhook listening on http://localhost:3000/"));
