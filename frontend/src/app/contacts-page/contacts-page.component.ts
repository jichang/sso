import { Component, OnInit } from '@angular/core';
import { Contact, ContactModelService } from '../contact-model.service';
import { session } from '../model';

@Component({
  selector: 'contacts-page',
  templateUrl: './contacts-page.component.html',
  styleUrls: ['./contacts-page.component.css']
})
export class ContactsPageComponent implements OnInit {
  contacts: Contact[] = [];

  constructor(private contactModel: ContactModelService) { }

  ngOnInit() {
    this.contactModel.contacts.subscribe(contacts => {
        this.contacts = contacts;
      });

    this.contactModel.select();
  }

  openCreateModal() {}
}
