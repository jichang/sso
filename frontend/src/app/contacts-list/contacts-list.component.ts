import { Component, Input } from '@angular/core';
import { Contact } from '../contact-model.service';

@Component({
  selector: 'contacts-list',
  templateUrl: './contacts-list.component.html',
  styleUrls: ['./contacts-list.component.css']
})
export class ContactsListComponent {
  @Input() contacts: Contact[];
}
