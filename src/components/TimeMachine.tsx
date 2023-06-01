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
  const [isOpen, setIsOpen] = useState(false);
  let date: string;
  let time: string;

  function openModal() {
    setIsOpen(true);
  }

  function closeModal() {
    setIsOpen(false);
  }

  function handleDateChange(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    date = event.target.value;
  }

  function handleTimeChange(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    time = event.target.value;
  }

  function submit() {
    const dateTimeString = `${date}T${time}`;
    const epochSeconds = Date.parse(dateTimeString);
    localStorage.setItem("untilDate", epochSeconds.toString());
    location.reload();
  }

  return (
    <div className="block">
      <button
        onClick={openModal}
        className="rounded-full aspect-square fixed top-1 right-1"
      >
        <img src="/tabler-icons/calendar.svg" />
      </button>

      <Modal
        isOpen={isOpen}
        onRequestClose={closeModal}
        style={modalStyle}
        contentLabel="入力メニュー"
      >
        <input type="date" onChange={handleDateChange} />
        <input type="time" onChange={handleTimeChange} />
        <button onClick={submit}>決定</button>
      </Modal>
    </div>
  );
}

export default TimeMachine;
