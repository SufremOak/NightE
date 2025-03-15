import express, { Request, Response } from 'express';
import path from 'path';

const app = express();
const port = 3000;

app.use('/static', express.static(path.join(__dirname, 'static')));

app.get('/', (req: Request, res: Response) => {
  res.sendFile(path.join(__dirname, 'index.html'));
});

app.listen(port, () => {
  console.log(`NightE frontend running at http://localhost:${port}`);
});