import { useState } from "react";
import Modal from "react-modal";

Modal.setAppElement("#root");

const modalStyle = {
  overlay: {
    background: "rgba(0, 0, 0, 0.5)",
  },
  content: {
    border: "none",
    background: "white",
    borderRadius: "2em",
    width: "19em",
    margin: "auto",
    height: "11em",
  },
};

function TimeMachine() {
  const [isOpen, toggleIsOpen] = useState(false);
  let date: string;
  let time: string;

  function toggleModal() {
    toggleIsOpen(!isOpen);
  }

  function changeDate(e: React.ChangeEvent<HTMLInputElement>) {
    date = e.target.value;
  }

  function changeTime(e: React.ChangeEvent<HTMLInputElement>) {
    time = e.target.value;
  }

  function submit() {
    const dateTime = `${date}T${time}`;
    const epochSeconds = Date.parse(dateTime);
    localStorage.setItem("untilDate", epochSeconds.toString());
    location.reload();
  }

  return (
    <div className="block">
      <button
        onClick={toggleModal}
        className="rounded-full aspect-square fixed top-1 right-1"
      >
        <img src="/tabler-icons/calendar.svg" />
      </button>

      <Modal
        isOpen={isOpen}
        onRequestClose={toggleModal}
        style={modalStyle}
        contentLabel="入力メニュー"
      >
        <input type="date" onChange={changeDate} />
        <input type="time" onChange={changeTime} />
        <button onClick={submit}>決定</button>
      </Modal>
    </div>
  );
}

export default TimeMachine;
