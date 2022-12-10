'use client';
import { Board } from '@lib/TileGame';

export default function BoardGame() {
  return (
    <div className="flex flex-col p-8 gap-4">
      <h1 className="text-2xl text-center font-bold">3x3 Tiles Solver</h1>
      <div className="flex w-full justify-center">
        <Board name="Generate and test" solutionMethod='generate_and_test' />
        <div className="divider divider-horizontal">OR</div>
        <Board name="Hill climbing" solutionMethod='hill_climb' />
      </div>
    </div>
  );
}
