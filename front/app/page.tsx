import Link from 'next/link';
import { FC } from 'react';

export default function Page() {
  return (
    <div className="flex flex-wrap h-full align-center justify-center gap-4">
      <AssignmentsCard
        title="Board Game"
        description="3x3 tiles solver using generate and test and hill climbing algorithms"
        buttonLabel="Board Game"
        link="board-game"
      />
    </div>
  );
}

export type AssignmentsCardType = {
  title: string;
  description: string;
  buttonLabel: string;
  link: string;
  imageSrc?: string;
};

const AssignmentsCard: FC<AssignmentsCardType> = ({
  title,
  description,
  buttonLabel,
  link,
  imageSrc,
}) => {
  return (
    <div className="card w-96 bg-base-100 shadow-xl image-full">
      <figure>
        <img
          src={imageSrc || 'https://placeimg.com/400/225/arch'}
          alt="Shoes"
        />
      </figure>
      <div className="card-body">
        <h2 className="card-title">{title}</h2>
        <p>{description}</p>
        <div className="card-actions justify-end">
          <Link href={link}>
            <button className="btn btn-primary">{buttonLabel}</button>
          </Link>
        </div>
      </div>
    </div>
  );
};
