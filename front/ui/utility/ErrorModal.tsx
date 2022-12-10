import { FC } from 'react';

export type ErrorModalProps = {
  message?: string;
};

const ErrorModal: FC<ErrorModalProps> = ({ message }) => {
  return (
    <>
      <input type="checkbox" id="error-modal" className="modal-toggle" />
      <div className="modal">
        <div className="modal-box relative gap-4">
          <label
            htmlFor="error-modal"
            className="btn btn-sm btn-circle absolute right-2 top-2"
          >
            ✕
          </label>
          <h3 className="p-0 m-0">Error!</h3>
          {message && <p className="p-0 m-0">{message}</p>}
        </div>
      </div>
    </>
  );
};

export default ErrorModal;
