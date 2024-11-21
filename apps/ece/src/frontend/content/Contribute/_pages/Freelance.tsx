import React, { useState } from 'react';
import Modal from '../../Utils/_components/Modal';
import Button from '../../Utils/_components/Button';
import styles from './Freelance.module.scss';

const Freelance: React.FC = () => {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [formData, setFormData] = useState({
    requesterName: '',
    email: '',
    phoneNumber: '',
    projectName: '',
    projectDescription: '',
    budget: '',
    deadline: '',
  });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Logic to handle form submission and quote generation
    console.log('Form submitted:', formData);
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  return (
    <div className={styles.freelanceContainer}>
      <h1>Freelance Project Quote</h1>
      <form onSubmit={handleSubmit} className={styles.freelanceForm}>
        <label>
          Your Name:
          <input
            type="text"
            name="requesterName"
            value={formData.requesterName}
            onChange={handleInputChange}
            required
          />
        </label>
        <label>
          Email:
          <input
            type="email"
            name="email"
            value={formData.email}
            onChange={handleInputChange}
            required
          />
        </label>
        <label>
          Phone Number (Optional):
          <input
            type="tel"
            name="phoneNumber"
            value={formData.phoneNumber}
            onChange={handleInputChange}
          />
        </label>
        <label>
          Project Name:
          <input
            type="text"
            name="projectName"
            value={formData.projectName}
            onChange={handleInputChange}
            required
          />
        </label>
        <label>
          Project Description:
          <textarea
            name="projectDescription"
            value={formData.projectDescription}
            onChange={handleInputChange}
            required
          />
        </label>
        <label>
          Budget:
          <input
            type="number"
            name="budget"
            value={formData.budget}
            onChange={handleInputChange}
            required
          />
        </label>
        <label>
          Deadline:
          <input
            type="date"
            name="deadline"
            value={formData.deadline}
            onChange={handleInputChange}
            required
          />
        </label>
        <Button label="Submit" onClick={handleSubmit} />
      </form>

      <Modal isOpen={isModalOpen} onClose={closeModal} title="Quote Submitted">
        <p>Your project quote request has been submitted successfully!</p>
        <Button label="Close" onClick={closeModal} />
      </Modal>
    </div>
  );
};

export default Freelance;