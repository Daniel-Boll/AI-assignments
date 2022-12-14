import { shuffle, sleep } from '@lib/utils';
import ErrorModal from '@ui/utility/ErrorModal';
import axios from 'axios';
import { FC, useRef, useState } from 'react';

interface Tile {
  content: string;
}

interface BoardProps {
  name: string;
  solutionMethod: 'generate_and_test' | 'hill_climbing';
}

export const Board: FC<BoardProps> = ({ name, solutionMethod }) => {
  const initialState = Array(8)
    .fill(0)
    .map((_, i) => ({ content: `${i + 1}` }));

  const [tiles, setTiles] = useState<Tile[]>([
    ...initialState,
    { content: '' },
  ]);
  const [loadingSolution, setLoadingSolution] = useState('');
  const [solutionSteps, setSolutionSteps] = useState<Tile[][]>([]);
  const [currentStep, setCurrentStep] = useState(0);
  const [speed, setSpeed] = useState(125);
  const [wasSolved, setWasSolved] = useState<boolean | undefined>();
  const [isRunning, setIsRunning] = useState(false);
  const stopAnimation = useRef(false);

  const solve = async () => {
    setLoadingSolution('loading');

    try {
      const resp = await axios.post(
        `${process.env.NEXT_PUBLIC_API_URL}/tiles/solve`,
        {
          method: solutionMethod,
          board: tiles,
        },
      );

      let solution: Tile[][] = resp.data?.solution_steps || [];
      solution?.unshift(tiles);

      verifySolution(solution);
      setSolutionSteps(solution);
      setLoadingSolution('');
      await animateSolution(solution);
    } catch (err) {
      // FIXME: Use error handling from nextjs 13 https://beta.nextjs.org/docs/routing/error-handling
      console.error('err at Board.solve: ', err);
      document.getElementById('error-modal').checked = true;
    }
    setLoadingSolution('');
  };

  const verifySolution = (_solutionSteps: Tile[][]) => {
    const lastStep = _solutionSteps?.[_solutionSteps?.length - 1] || [];
    // deep clone initialState
    const solution = JSON.parse(JSON.stringify(initialState));
    solution.push({ content: '' });

    setWasSolved(JSON.stringify(lastStep) === JSON.stringify(solution));
  };

  const animateSolution = async (steps: Tile[][]) => {
    setIsRunning(true);

    for (const [idx, step] of steps?.entries()) {
      setTiles(step);
      setCurrentStep(idx);

      if (stopAnimation.current) break;

      await sleep(speed);
    }

    stopAnimation.current = false;
    setIsRunning(false);
  };

  const handleChangeStep = (nextStep: number) => {
    if (nextStep - 1 < 0 || nextStep > solutionSteps.length) return;
    setTiles(solutionSteps[nextStep - 1]);
    setCurrentStep(nextStep - 1);
  };

  const handleShuffle = () => {
    const currentState = [...tiles];
    const shuffled = shuffle(currentState);
    setTiles(shuffled);
  };

  return (
    <div className="flex flex-col justify-center gap-2 w-full prose">
      <h4 className="text-center">{name}</h4>
      <div className="flex flex-row justify-between items-center px-4 gap-2">
        <div className="flex gap-1 flex-col">
          <div className="flex">
            <p className="m-0 mr-2">current step:</p>
            <input
              type="number"
              min={1}
              max={solutionSteps.length}
              value={currentStep + 1}
              onChange={({ target: { value } }) => handleChangeStep(+value)}
              className="input input-bordered input-sm w-16 pr-0"
            />
          </div>
          <div className="flex gap-2">
            <p className="m-0">speed (ms):</p>
            <input
              className="input input-bordered input-sm w-16"
              value={speed}
              onChange={({ target: { value } }) => setSpeed(+value)}
            />
          </div>
        </div>
        {typeof wasSolved === 'boolean' && (
          <div className="flex gap-2">
            {!wasSolved ? (
              <p className="text-red-500 font-bold">Any solution found</p>
            ) : (
              <p className="text-green-500 font-bold">Solution found</p>
            )}
          </div>
        )}
        <div className="flex gap-2 flex-col">
          <span>total steps: {solutionSteps.length}</span>
          <div className="flex gap-2 justify-between">
            <button
              disabled={
                loadingSolution === 'loading' || solutionSteps.length === 0
              }
              className="btn btn-outline btn-sm normal-case"
              onClick={() => animateSolution(solutionSteps)}
            >
              run
            </button>
            <button
              disabled={!isRunning}
              className="btn btn-outline btn-error btn-sm normal-case"
              onClick={() => (stopAnimation.current = true)}
            >
              stop
            </button>
          </div>
        </div>
      </div>
      <div className="grid grid-cols-3 gap-2 bg-neutral rounded-xl p-2">
        {tiles.map((tile, index) => (
          <div
            key={`${tile}${index}`}
            className={
              tile.content !== ''
                ? 'bg-neutral-focus rounded-xl text-center text-neutral-content p-2'
                : ''
            }
          >
            {tile.content}
          </div>
        ))}
      </div>
      <div className="btn-group">
        <button
          className={`btn btn-primary w-1/2 ${loadingSolution}`}
          onClick={solve}
        >
          Solve
        </button>
        <button className="btn w-1/2" onClick={handleShuffle}>
          Shuffle
        </button>
      </div>
      <ErrorModal message="Error calculating solution!" />
    </div>
  );
};
