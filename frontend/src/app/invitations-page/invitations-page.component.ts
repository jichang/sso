import { Component, OnInit } from "@angular/core";
import { session } from "../model";
import {
  Invitation,
  InvitationModelService
} from "../invitation-model.service";
import { Subscription } from "rxjs";
import { Router } from "@angular/router";
import { MatSnackBar } from "@angular/material";

@Component({
  selector: "invitations-page",
  templateUrl: "./invitations-page.component.html",
  styleUrls: ["./invitations-page.component.css"]
})
export class InvitationsPageComponent implements OnInit {
  invitations: Invitation[] = [];
  invitationsSubscription: Subscription;

  constructor(
    private router: Router,
    private invitationModel: InvitationModelService,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit() {
    this.invitationsSubscription = this.invitationModel.invitations.subscribe(
      invitations => {
        this.invitations = invitations;
      },
      err => {
        if (err.status === 403) {
          this.snackBar.open("action is forbidden", "Dismiss", {
            duration: 3000
          });
        }
      }
    );

    let currUser = session.currUser();
    if (currUser) {
      this.invitationModel.select(currUser.id);
    } else {
      this.router.navigate(["signin"]);
    }
  }

  ngOnDestroy() {
    this.invitationsSubscription.unsubscribe();
  }

  createInvitation() {
    let currUser = session.currUser();
    if (currUser) {
      this.invitationModel.create(currUser.id).subscribe(
        invitation => {
          this.snackBar.open("Invitation created", "Dismiss", {
            duration: 3000
          });
        },
        err => {
          if (err.status === 403) {
            this.snackBar.open("action is forbidden", "Dismiss", {
              duration: 3000
            });
          }
        }
      );
    }
  }

  removeInvitation(invitation: Invitation) {
    let currUser = session.currUser();
    if (currUser) {
      this.invitationModel.remove(currUser.id, invitation).subscribe(
        invitation => {
          this.snackBar.open("Invitation removed", "Dismiss", {
            duration: 3000
          });
        },
        err => {
          if (err.status === 403) {
            this.snackBar.open("action is forbidden", "Dismiss", {
              duration: 3000
            });
          }
        }
      );
    }
  }
}
