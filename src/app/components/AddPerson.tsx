"use client";

import styles from '../page.module.css';
import { useState, ChangeEvent } from "react";
import { PersonForCreate } from "@/bindings";
import { personController } from "@/api/api";

interface Props {
    onCreate: () => void
}

export const AddPerson = ({ onCreate }: Props) => {
    const emptyPerson: PersonForCreate = { title: "", name: { first: "", last: "" }, marketing: false };
    const [person, setPerson] = useState<PersonForCreate>(emptyPerson);

    const handleInputChange = (event: ChangeEvent<HTMLInputElement>): void => {
        let value = event.target.value;
        switch (event.target.name) {
            case "title": setPerson({ ...person, title: value }); break;
            case "firstName": setPerson({ ...person, name: { ...person.name, first: value } }); break;
            case "lastName": setPerson({ ...person, name: { ...person.name, last: value } }); break;
            case "marketing": {
                setPerson({ ...person, marketing: !person.marketing });
                break;
            }
            default: console.error("target name does not match any property of person"); break;
        }
    }

    const addPerson = async () => {
        try {
            let person_id = await personController.create(person);
            onCreate();
            setPerson(emptyPerson);
        } catch (e) {
            console.error("error = ", e)
        }
    }

    return (
        <div className={styles.container}>
            <form className={styles.formDiv} onSubmit={(e) => { e.preventDefault(); }}>
                <h2>Add a new person</h2>
                <input
                    className={styles.textInput}
                    type="text"
                    placeholder='Title...'
                    name='title'
                    value={person.title}
                    onChange={handleInputChange}
                />
                <input
                    className={styles.textInput}
                    type="text"
                    placeholder='First Name...'
                    name='firstName'
                    value={person.name.first}
                    onChange={handleInputChange}
                />
                <input
                    className={styles.textInput}
                    type="text"
                    placeholder='Last Name...'
                    name='lastName'
                    value={person.name.last}
                    onChange={handleInputChange}
                />
                <div className={styles.checkboxContainer}>
                    <input
                        className={styles.checkboxInput}
                        type="checkbox"
                        placeholder='Marketing'
                        name='marketing'
                        id='marketing'
                        checked={person.marketing}
                        onChange={handleInputChange} />
                    <label htmlFor="marketing" className={styles.checkboxLabel}>Marketing</label>
                </div>
                <button className={styles.submitButton} onClick={() => addPerson()}>Add Person</button>
            </form>
        </div >
    )
}
