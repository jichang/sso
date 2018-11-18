import { Component, OnInit, OnDestroy } from "@angular/core";
import { Router } from "@angular/router";
import { Contact, ContactModelService } from "../contact-model.service";
import { session } from "../model";
import { ContactAction } from "../contacts-list/contacts-list.component";
import { Subscription } from "rxjs";
import { MatDialog, MatDialogRef } from "@angular/material";
import { ConfirmDialogComponent } from "../confirm-dialog/confirm-dialog.component";

@Component({
  selector: "contacts-page",
  templateUrl: "./contacts-page.component.html",
  styleUrls: ["./contacts-page.component.css"]
})
export class ContactsPageComponent implements OnInit, OnDestroy {
  contacts: Contact[] = [];
  subscription: Subscription;
  dialogRef: MatDialogRef<ConfirmDialogComponent>;

  constructor(
    private router: Router,
    private contactModel: ContactModelService,
    public dialog: MatDialog
  ) {}

  ngOnInit() {
    this.subscription = this.contactModel.contacts.subscribe(contacts => {
      this.contacts = contacts;
    });

    let currUser = session.currUser();
    if (currUser) {
      this.contactModel.select(currUser.id);
    } else {
      this.router.navigate(["signin"]);
    }
  }

  ngOnDestroy() {
    this.subscription.unsubscribe();
  }

  handleAction($event: ContactAction) {
    let { type, contact } = $event;

    switch (type) {
      case "delete":
        this.dialogRef = this.dialog.open(ConfirmDialogComponent, {
          height: "400px",
          width: "600px",
          data: {
            title: "Delete Contact?",
            message: "delete contact " + contact.identity
          }
        });
        this.dialogRef.afterClosed().subscribe(result => {
          if (result) {
            this.contactModel.remove(contact).subscribe((contact: Contact) => {
              console.log(contact);
            });
          }
        });
        break;
    }
  }
}
