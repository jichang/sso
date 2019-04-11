import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { InvitationsListComponent } from './invitations-list.component';

describe('InvitationsListComponent', () => {
  let component: InvitationsListComponent;
  let fixture: ComponentFixture<InvitationsListComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ InvitationsListComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(InvitationsListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
