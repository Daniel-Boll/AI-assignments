import React, { FC, useState } from 'react';

interface Tile {
  content: string;
}

const Board: FC<any> = ({}) => {
  const initialState = Array(8)
    .fill(0)
    .map((_, i) => ({ content: i.toString() }));

  const [tiles, _] = useState<Tile[]>([...initialState, { content: '' }]);

  return (
    <div className="grid grid-cols-3 gap-2">
      {tiles.map((tile, index) => (
        <div key={`${tile}${index}`} className="bg-gray-200 p-2">
          {tile.content}
        </div>
      ))}
    </div>
  );
};

export { Board };
