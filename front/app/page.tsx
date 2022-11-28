// import { Board } from '../lib/TileGame';
'use client';
import { shuffle } from '@lib/utils/array';
import React, { FC, useState } from 'react';

interface Tile {
  content: string;
}

interface BoardProps {
  name: string;
}

const Board: FC<BoardProps> = ({ name }) => {
  const initialState = Array(8)
    .fill(0)
    .map((_, i) => ({ content: `${i + 1}` }));

  const [tiles, setTiles] = useState<Tile[]>([
    ...initialState,
    { content: '' },
  ]);

  const solve = async () => {
    const request = await fetch('localhost:8080/tiles/solve', {
      method: 'POST',
      body: JSON.stringify(tiles),
    });
    const result = await request.json();

    console.log(result);
  };

  const handleShuffle = () => {
    const currentState = [...tiles];
    const shuffled = shuffle(currentState);
    setTiles(shuffled);
  };

  return (
    <div className="flex flex-col justify-center gap-2 w-full prose">
      <h4 className="text-center">{name}</h4>
      <div className="grid grid-cols-3 gap-2 bg-neutral rounded-xl p-2">
        {tiles.map((tile, index) => (
          <div
            key={`${tile}${index}`}
            className="bg-neutral-focus rounded-xl text-center text-neutral-content p-2"
          >
            {tile.content}
          </div>
        ))}
      </div>
      <div className="btn-group">
        <button className="btn btn-primary w-1/2" onClick={solve}>
          Solve
        </button>
        <button className="btn w-1/2" onClick={handleShuffle}>
          Shuffle
        </button>
      </div>
    </div>
  );
};

export default function Page() {
  return (
    <div className="flex flex-col p-8 gap-4">
      <h1 className="text-2xl text-center font-bold">3x3 Tiles Solver</h1>
      <div className="flex w-full justify-center">
        <Board name="Generate and test" />
        <div className="divider divider-horizontal">OR</div>
        <Board name="Hill climbing" />
      </div>
    </div>
  );
}
