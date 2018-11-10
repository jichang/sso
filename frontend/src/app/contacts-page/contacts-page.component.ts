import { Component, OnInit, OnDestroy } from "@angular/core";
import { Router } from "@angular/router";
import { Contact, ContactModelService } from "../contact-model.service";
import { session } from "../model";
import { ContactAction } from "../contacts-list/contacts-list.component";
import { Subscription } from "rxjs";

@Component({
  selector: "contacts-page",
  templateUrl: "./contacts-page.component.html",
  styleUrls: ["./contacts-page.component.css"]
})
export class ContactsPageComponent implements OnInit, OnDestroy {
  contacts: Contact[] = [];
  subscription: Subscription;

  constructor(
    private router: Router,
    private contactModel: ContactModelService
  ) {}

  ngOnInit() {
    this.subscription = this.contactModel.contacts.subscribe(contacts => {
      this.contacts = contacts;
    });

    let currUser = session.currUser();
    if (currUser) {
      this.contactModel.select(currUser.id);
    } else {
      this.router.navigate(["login"]);
    }
  }

  ngOnDestroy() {
    this.subscription.unsubscribe();
  }
  openCreateModal() {}

  handleAction($event: ContactAction) {
    let { type, contact } = $event;

    switch (type) {
      case "delete":
        this.contactModel.remove(contact).subscribe((contact: Contact) => {
          console.log(contact);
        });
        break;
    }
  }
}
