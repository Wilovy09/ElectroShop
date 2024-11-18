import Swal from "sweetalert2";

const showConfirmationSwal = async (title: string, text: string) => {
  let isConfirmed: boolean | null = null;
  await Swal.fire({
    title: title,
    text: text,
    icon: "warning",
    showCancelButton: true,
    confirmButtonColor: "red",
    confirmButtonText: "Confirmar",
    cancelButtonText: "Cancelar",
    reverseButtons: true,
    customClass: {
      popup: "colored-toast",
      title: "colored-modal-title",
      htmlContainer: "colored-modal-text",
      confirmButton: "colored-modal-button",
      cancelButton: "colored-modal-button",
    },
  }).then((result) => {
    if (result.isConfirmed) {
      isConfirmed = true;
    } else {
      isConfirmed = false;
    }
  });
  return isConfirmed;
};

const style = document.createElement("style");
style.textContent = `
  .colored-toast {
    background-color: #1e293b !important;
    color: white !important;
  }
  
  .colored-modal-title {
    color: white !important;
  }
  
  .colored-modal-text {
    color: #e2e8f0 !important;
  }
`;
document.head.appendChild(style);

export default showConfirmationSwal;
