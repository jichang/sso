import { Component, Input, Output, EventEmitter } from "@angular/core";
import { Contact } from "../contact-model.service";

export type ContactActionType = "delete";

export interface ContactAction {
  type: ContactActionType;
  contact: Contact;
}

@Component({
  selector: "contacts-list",
  templateUrl: "./contacts-list.component.html",
  styleUrls: ["./contacts-list.component.css"]
})
export class ContactsListComponent {
  @Input() contacts: Contact[];
  @Output() action = new EventEmitter<ContactAction>();

  remove(contact: Contact) {
    this.action.emit({
      type: "delete",
      contact
    });
  }
}
